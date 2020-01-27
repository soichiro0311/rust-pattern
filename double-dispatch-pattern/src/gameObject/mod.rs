pub struct Rock {}

impl GameObject for Rock {
    fn beats(&self, obj: impl GameObject) -> bool {
        return !obj.beatsRock();
    }

    fn beatsPaper(&self) -> bool {
        false
    }

    fn beatsRock(&self) -> bool {
        false
    }

    fn beatsScissors(&self) -> bool {
        true
    }
}

impl Rock {
    pub fn new() -> Rock {
        Rock {}
    }
}

pub struct Scissors {}

impl GameObject for Scissors {
    fn beats(&self, obj: impl GameObject) -> bool {
        return !obj.beatsScissors();
    }

    fn beatsPaper(&self) -> bool {
        true
    }

    fn beatsRock(&self) -> bool {
        false
    }

    fn beatsScissors(&self) -> bool {
        false
    }
}

impl Scissors {
    pub fn new() -> Scissors {
        Scissors {}
    }
}

pub struct Paper {}

impl GameObject for Paper {
    fn beats(&self, obj: impl GameObject) -> bool {
        return !obj.beatsPaper();
    }

    fn beatsPaper(&self) -> bool {
        false
    }

    fn beatsRock(&self) -> bool {
        true
    }

    fn beatsScissors(&self) -> bool {
        false
    }
}

impl Paper {
    pub fn new() -> Paper {
        Paper {}
    }
}

pub trait GameObject {
    fn beats(&self, obj: impl GameObject) -> bool;
    fn beatsRock(&self) -> bool;
    fn beatsPaper(&self) -> bool;
    fn beatsScissors(&self) -> bool;
}
