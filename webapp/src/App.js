import "./App.css";
import React, { useState } from "react";

import { HexColorPicker } from "react-colorful";

const ColourPicker = () => {
  const [color, setColor] = useState("#aabbcc");
  return (
    <div class="ColourPicker">
      <HexColorPicker color={color} onChange={setColor} />{" "}
      <button onClick={() => send_colour(color)}> Update</button>
    </div>
  );
};

class ModeSelect extends React.Component {
  constructor(props) {
    super(props);
    this.state = "fixed"; //"solid", "block", "individual"
  }

  render() {
    return (
      <div class="mode">
        <button
          onClick={() =>
            fetch("/api/rainbow").then((data) => console.log(data))
          }
        >
          Rainbow
        </button>
        <button
          onClick={() => fetch("/api/fade").then((data) => console.log(data))}
        >
          Fade
        </button>
        <button />
      </div>
    );
  }
}

class Switch extends React.Component {
  render() {
    return (
      <div class="Switch">
        <button
          onClick={() => fetch("/api/on").then((data) => console.log(data))}
        >
          ON
        </button>
        <button
          onClick={() => fetch("/api/off").then((data) => console.log(data))}
        >
          OFF
        </button>
      </div>
    );
  }
}

class App extends React.Component {
  render() {
    return (
      <div class="App">
        <Switch />
        <ColourPicker />
        <ModeSelect />
      </div>
    );
  }
}

function send_colour(colour) {
  let route = `/api/colour/${colour.slice(1)}`;
  //console.log(route);
  fetch(route).then((data) => console.log(data));
}

export default App;
