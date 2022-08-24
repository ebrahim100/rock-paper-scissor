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
      user: Game,
      computer: Game,
      result: Hand,
      round: i32,
  }
