use sodiumoxide::crypto::generichash::State;

pub fn generic_state() -> Result<State, ()> {
    let hasher = State::new(32, None)?;
    Ok(hasher)
}

pub fn generic_finalize(hasher: State) -> Result<Vec<u8>, ()> {
    let finalized = hasher.finalize()?;

    Ok(Vec::from(finalized.as_ref()))
}

pub fn raw_generic(bytes: &[u8]) -> Result<Vec<u8>, ()> {
    let mut hasher = generic_state()?;
    hasher.update(bytes)?;

    Ok(generic_finalize(hasher)?)
}
