import React, { useState } from "react";
import Game from "./Game";
import Intro from "./Intro";
import { Phase } from "./types";

export default function App() {
  const [phase, setPhase] = useState<Phase>({cur: 'intro'});

  switch (phase.cur) {
  case 'intro':
    return <Intro {...{setPhase}} />;
  case 'game': {
    const {bot} = phase;
    return <Game {...{setPhase, bot}} />;
  }
  default:
    throw new Error('bad phase');
  }
}