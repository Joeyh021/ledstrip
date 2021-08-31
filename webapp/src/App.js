import "./App.css";
import React, { useState } from "react";

import { HexColorPicker } from "react-colorful";

const ColourPicker = () => {
  const [color, setColor] = useState("#aabbcc");
  return <HexColorPicker color={color} onChange={setColor} />;
};

function App() {
  return (
    <div className="App" class="center">
      <ColourPicker colour="ffffff" />
    </div>
  );
}

export default App;
