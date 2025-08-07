use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UscFile {
    pub usc: UscData,
    pub version: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UscData {
    pub objects: Vec<UscObject>,
    pub offset: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuidePoint {
    pub beat: f64,
    pub ease: String,
    pub lane: f64,
    pub size: f64,
    pub timeScaleGroup: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
        #[serde(skip_serializing_if = "Option::is_none")]
        direction: Option<String>,
        lane: f64,
        size: f64,
        timeScaleGroup: usize,
        trace: bool,
    },
    
    #[serde(rename = "slide")]
    Slide {
        connections: Vec<SlideConnection>,
        critical: bool,
    },

    #[serde(rename = "damage")]
    Damage {
        beat: f64,
        lane: f64,
        size: f64,
        timeScaleGroup: usize, 
    },

    #[serde(rename = "guide")]
    Guide {
        color: String,
        fade: String,
        midpoints: Vec<GuidePoint>,
    },

    // レーンイベント　未実装 

    // #[serde(rename = "laneEvent")]
    // LaneEvent {
    //     beat: f64,
    //     alpha: Option<f64>,
    //     rolation: Option<f64>,
    // },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TimeScaleChange {
    pub beat: f64,
    pub timeScale: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SlideConnection {
    pub beat: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub critical: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ease: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub judgeType: Option<String>,
    pub lane: f64,
    pub size: f64,
    pub timeScaleGroup: usize,
    #[serde(rename = "type")]
    pub r#type: String,
}

// レーンイベントの定義
// まだエンジン側で実装されていないためコメントアウト

// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct LaneEvent {
//     pub beat: f64,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub alpha: Option<f64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
// 　　pub rolation: Option<f64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
// }