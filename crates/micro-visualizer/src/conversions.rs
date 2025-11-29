pub fn seconds_to_frames(seconds: f64, frame_rate: u64) -> u64 {
	(seconds * frame_rate as f64) as u64
}

pub fn seconds_to_frames_i64(seconds: f64, frame_rate: u64) -> i64 {
	(seconds * frame_rate as f64) as i64
}

pub fn frame_to_seconds(frame: u64, frame_rate: u64) -> f64 {
	frame as f64 / frame_rate as f64
}
