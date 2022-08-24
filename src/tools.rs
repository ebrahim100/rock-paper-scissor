  #[derive(Debug)]
 pub  enum Game {
    STONE,
    PAPER,
    SCISSOR,
  }

  #[derive(Debug)]
  pub enum Hand {
      WON,
      LOST,
      DRAW,
  }

  #[derive(Debug)]
  pub struct Result {
    pub user: Game,
    pub computer: Game,
    pub result: Hand,
    pub round: i32,
  }
