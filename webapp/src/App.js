import "./App.css";
import React, { useState } from "react";

import { HexColorPicker } from "react-colorful";

const ColourPicker = () => {
  const [color, setColor] = useState("#aabbcc");
  return (
    <div>
      <HexColorPicker color={color} onChange={setColor} />{" "}
      <button onClick={() => send_colour(color)}> Update</button>
    </div>
  );
};

function App() {
  return (
    <div className="App">
      <ColourPicker />
    </div>
  );
}

function send_colour(colour) {
  fetch("/on")
    .then((response) => response.json())
    .then((data) => console.log(data));
}
export default App;
