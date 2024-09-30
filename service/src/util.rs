struct Construct {
    val: u128,
    unit: String,
}

const SEC_MS: u128 = 1000;
const MIN_MS: u128 = SEC_MS * 60;
const HOUR_MS: u128 = MIN_MS * 60;

pub fn ms_to_str(ms: u128) -> String {
    let hours = Construct {
        val: ms / HOUR_MS,
        unit: "h".to_string(),
    };
    let mut ms = ms - hours.val * HOUR_MS;

    let mins = Construct {
        val: ms / MIN_MS,
        unit: "min".to_string(),
    };
    ms -= mins.val * MIN_MS;

    let seconds = Construct {
        val: ms / SEC_MS,
        unit: "s".to_string(),
    };

    let vec = vec![hours, mins, seconds];
    return vec
        .iter()
        .filter(|v| v.val > 0)
        .map(|v| format!("{}{}", v.val, v.unit))
        .collect::<Vec<String>>()
        .join(" ");
}
