//! We implement perfect negotiation in order to try to
//! reduce the number of errors that can occur when streams
//! and conditions change. In this case, the server is
//! considered the impolite peer.
//!
//! https://w3c.github.io/webrtc-pc/#perfect-negotiation-example

use std::{
    pin::Pin,
    sync::atomic::{AtomicBool, Ordering},
};

use anyhow::Result;
use futures::Future;
use log::warn;
use webrtc::{
    ice_transport::ice_candidate::RTCIceCandidateInit,
    peer_connection::{
        sdp::{sdp_type::RTCSdpType, session_description::RTCSessionDescription},
        signaling_state::RTCSignalingState,
    },
};
use api::media_type::MediaType;
use api::negotiation::Negotiation;
//use crate::signaling::packets::{MediaType, Negotiation};

use super::Peer;

/// Current negotiation state
#[derive(Default)]
pub struct NegotiationState {
    making_offer: AtomicBool,
    ignore_offer: AtomicBool,
    is_setting_remote_answer_pending: AtomicBool,
}

/// Negotiation function
pub type NegotiationFn = Box<
    dyn (Fn(Negotiation) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'static>>)
        + Send
        + Sync,
>;

impl Peer {
    /// Renegotiate the current connection
    pub async fn renegotiate(&self) -> Result<()> {
        // TODO: not sure if required
        // ignore for now

        /* // Signal that we are currently creating an offer
        self.negotiation_state
            .making_offer
            .store(true, Ordering::SeqCst);

        // Create an offer
        let offer = self.connection.create_offer(None).await?;

        // Set the local description based on this offer
        self.connection.set_local_description(offer.clone()).await?;

        // Send an answer back
        (self.negotation_fn)(Negotiation::SDP { description: offer }).await?;

        // Mark as complete
        self.negotiation_state
            .making_offer
            .store(false, Ordering::SeqCst); */

        Ok(())
    }

    /// Add media types to the media type buffer
    pub async fn extend_media_type_buffer(&self, buffer: Option<Vec<MediaType>>) {
        if let Some(buffer) = buffer {
            let mut media_type_buffer = self.media_type_buffer.lock().await;
            media_type_buffer.extend(buffer.into_iter());
        }
    }

    /// Consume a given RTC session description
    pub async fn consume_sdp(&self, description: RTCSessionDescription) -> Result<()> {
        // Check if we are ready to receive a new SDP
        let ready_for_offer = !self.negotiation_state.making_offer.load(Ordering::SeqCst)
            && (self.connection.signaling_state() == RTCSignalingState::Stable
                || self
                    .negotiation_state
                    .is_setting_remote_answer_pending
                    .load(Ordering::SeqCst));

        // Check if this offer is unexpected
        let sdp_type = description.sdp_type.clone();
        let offer_collision = sdp_type == RTCSdpType::Offer && !ready_for_offer;

        // We are the impolite peer hence we ignore the offer
        self.negotiation_state
            .ignore_offer
            .store(offer_collision, Ordering::Relaxed);

        if offer_collision {
            warn!("Unexpected offer received from the client!");
            return Ok(());
        }

        // If we received an answer, mark it as such locally
        self.negotiation_state
            .is_setting_remote_answer_pending
            .store(sdp_type == RTCSdpType::Answer, Ordering::SeqCst);

        // Apply the new remote description
        self.connection.set_remote_description(description).await?;

        // Restore the default value
        self.negotiation_state
            .is_setting_remote_answer_pending
            .store(false, Ordering::SeqCst);

        // If we received an offer, send an answer back
        if sdp_type == RTCSdpType::Offer {
            // Create an answer
            let answer = self.connection.create_answer(None).await?;

            // Set the local description based on this answer
            self.connection
                .set_local_description(answer.clone())
                .await?;

            // Send an answer back
            (self.negotiation_fn)(Negotiation::SDP {
                description: answer,
                media_type_buffer: None,
            })
            .await?;
        }

        Ok(())
    }

    /// Consume a given ICE candidate
    pub async fn consume_ice(&self, candidate: RTCIceCandidateInit) -> Result<()> {
        if let Err(error) = self.connection.add_ice_candidate(candidate).await {
            if !self.negotiation_state.ignore_offer.load(Ordering::SeqCst) {
                return Err(error.into());
            }
        }

        Ok(())
    }
}
