import React, { ChangeEvent,  useState } from "react";
import { Phase } from "./types";

import './intro.css';

interface IntroProps {
  setPhase: React.Dispatch<Phase>;
}
export default function Intro({setPhase}: IntroProps) {
  const [opponent, setOpponent] = useState('engine');
  const [level, setLevel] = useState(4);
  const [color, setColor] = useState('random');
  function startGame() {
    const playerIsWhite = color === 'random' ? Math.random() < 0.5 : color === 'white';
    const isWhite = !playerIsWhite;
    const bot = opponent === 'friend' ? undefined : { depth: level + 1, isWhite };
    setPhase({ cur: 'game', bot });
  }
  const radioProps = (value: string, setValue: React.Dispatch<string>, disabled=false) => (cur: string) =>
    ({
      value: cur,
      checked: cur === value,
      disabled,
      onChange(e: ChangeEvent<HTMLInputElement>) {
        setValue(e.target.value);
      }
    });
  const opponentProps = radioProps(opponent, setOpponent);
  const colorProps = radioProps(color, setColor, opponent !== 'engine');
  return <div className="intro">
    <h1>Rust Chess</h1>
    <div className="flex-col">
      <div>
        <label>
          <input type="radio" name="opponent" {...opponentProps('engine')} />
              vs. Engine{' '}
        </label>
        <label onClick={() => setOpponent('engine')}>
          level {level}
          <input type="range"
            disabled={opponent !== 'engine'}
            min={1} max={7}
            value={level}
            onChange={e => setLevel(+e.target.value)}/>
        </label>

        <div className="indent" onClick={() => setOpponent('engine')}>
              Play as:{' '}
          <label><input type="radio" name="color" {...colorProps('white')}/> White</label>
          <label><input type="radio" name="color" {...colorProps('black')}/> Black</label>
          <label><input type="radio" name="color" {...colorProps('random')}/> Random</label>
        </div>
      </div>
      <label>
        <input type="radio" name="opponent" {...opponentProps('friend')} />
          vs. Friend (local)
      </label>
      <div>
        <button className="start-btn" onClick={startGame}>Start</button>
      </div>
    </div>

  </div>;
}