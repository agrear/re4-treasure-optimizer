type Id = number;

type Gem = {
  id: Id,
  name: string,
  shape: Shape,
  color: Color,
  value: number
};

type Appraisal = {
  value: number,
  bonus: number
}

type Treasure = {
  id: Id,
  name: string,
  sockets: [number, number],
  value: number
};

type GemCollection = Id[];

type TreasureCollection = Id[];

type Socket = {
  shape: Shape,
  gemId: Id | null
};

type SocketedTreasure = {
  id: Id,
  sockets: Socket[],
  appraisal: Appraisal
};
