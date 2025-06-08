use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct UscFile {
    pub metadata: Metadata,
    pub content: Content,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub version: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub usc: UscContent,
    pub version: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UscContent {
    pub objects: Vec<UscObject>,
    pub offset: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum UscObject {
    #[serde(rename = "bpm")]
    Bpm {
        beat: f64,
        bpm: f64,
    },
    #[serde(rename = "timeScaleGroup")]
    TimeScaleGroup {
        changes: Vec<TimeScaleChange>,
    },
    #[serde(rename = "single")]
    Single {
        beat: f64,
        critical: bool,
        lane: f64,
        size: f64,
        timeScaleGroup: i32,
        trace: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        direction: Option<String>,
    },
    #[serde(rename = "slide")]
    Slide {
        connections: Vec<SlideConnection>,
        critical: bool,
    },
    #[serde(rename = "guide")]
    Guide {
        color: String,
        fade: String,
        midpoints: Vec<GuidePoint>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimeScaleChange {
    pub beat: f64,
    pub timeScale: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlideConnection {
    pub beat: f64,
    pub critical: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ease: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub judgeType: Option<String>,
    pub lane: f64,
    pub size: f64,
    pub timeScaleGroup: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "type")]
    pub connection_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GuidePoint {
    pub beat: f64,
    pub ease: String,
    pub lane: f64,
    pub size: f64,
    pub timeScaleGroup: i32,
}