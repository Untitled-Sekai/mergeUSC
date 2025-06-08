use std::error::Error;
mod r#type;
pub use r#type::*;

pub struct UscMerger;

impl UscMerger {
    pub fn merge(files: Vec<UscFile>) -> Result<UscFile, Box<dyn Error>> {
        if files.is_empty() {
            return Err("No files to merge".into());
        }

        if files.len() == 1 {
            return Ok(files[0].clone());
        }

        let mut base_file = files[0].clone();
        
        for i in 1..files.len() {
            base_file = Self::merge_two(base_file, files[i].clone())?;
        }

        Ok(base_file)
    }

    fn merge_two(base: UscFile, file_to_merge: UscFile) -> Result<UscFile, Box<dyn Error>> {
        let mut result = base;
        let mut time_scale_group_offset = count_time_scale_groups(&result);

        for object in &file_to_merge.usc.objects {
            match object {
                // BPM
                UscObject::Bpm { .. } => {
                    if result.usc.objects.iter().any(|obje| matches!(obje, UscObject::Bpm { .. })) {
                        continue; // Skip if BPM already exists
                    }
                    result.usc.objects.push(object.clone());
                }

                // Time scale group
                UscObject::TimeScaleGroup { changes } => {
                    result.usc.objects.push(UscObject::TimeScaleGroup {
                        changes: changes.clone(),
                    });
                }

                // single note
                UscObject::Single { beat, critical, lane, size, timeScaleGroup, trace } => {
                    result.usc.objects.push(UscObject::Single {
                        beat: *beat,
                        critical: *critical,
                        lane: *lane,
                        size: *size,
                        timeScaleGroup: timeScaleGroup + time_scale_group_offset,
                        trace: *trace,
                    });
                }

                // slide
                UscObject::Slide { connections, critical } => {
                    let adjusted_connections = connections.iter()
                        .map(|conn| SlideConnection {
                            beat: conn.beat,
                            critical: conn.critical,
                            ease: conn.ease.clone(),
                            judgeType: conn.judgeType.clone(),
                            lane: conn.lane,
                            size: conn.size,
                            timeScaleGroup: conn.timeScaleGroup + time_scale_group_offset,
                            r#type: conn.r#type.clone(),
                        })
                        .collect();
                    
                    result.usc.objects.push(UscObject::Slide {
                        connections: adjusted_connections,
                        critical: *critical,
                    });
                }  

                // Damage
                UscObject::Damage { beat, lane, size, timeScaleGroup } => {
                    result.usc.objects.push(UscObject::Damage {
                        beat: *beat,
                        lane: *lane,
                        size: *size,
                        timeScaleGroup: timeScaleGroup + time_scale_group_offset,
                    });
                } 

                UscObject::Guide { color, fade, midpoints } => {
                    let adjusted_midpoints = midpoints.iter()
                        .map(|point| GuidePoint {
                            beat: point.beat,
                            ease: point.ease.clone(),
                            lane: point.lane,
                            size: point.size,
                            timeScaleGroup: point.timeScaleGroup + time_scale_group_offset,
                        })
                        .collect();
                    
                    result.usc.objects.push(UscObject::Guide {
                        color: color.clone(),
                        fade: fade.clone(),
                        midpoints: adjusted_midpoints,
                    });
                }

                // _ => {
                //     result.usc.objects.push(object.clone());
                // }
            }
        }

        Ok(result)
    }
}

// --- utility functions ---

fn count_time_scale_groups(file: &UscFile) -> usize {
    file.usc.objects.iter()
        .filter(|obj| matches!(obj, UscObject::TimeScaleGroup { .. }))
        .count()
}