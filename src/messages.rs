pub enum PlayerInstructions {
    BeginIncreaseThrottle,
    BeginDecreaseThrottle,
    StopIncreaseThrottle,
    StopDecreaseThrottle,
    BeginRotateRight,
    BeginRotateLeft,
    StopRotateRight,
    StopRotateLeft,
    Fire,
    Collide,
    Collect,
    ShieldUp,
    ShieldDown,
}

pub enum GameInstructions {
    Fire,
    Explode,
    Trail,
    NewAsteroid,
    Collect
}
