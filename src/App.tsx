import React from "react";
import { invoke } from "@tauri-apps/api/tauri";

function App() {
  const [filePath, setFilePath] = React.useState<string | undefined>();

  const upload = async () => {
    try {
      await invoke("upload_file", { path: filePath });
    } catch (error) {
      setFilePath(undefined);
    }
  };

  return (
    <div className="container">
      <input value={filePath} onChange={(e) => setFilePath(e.target.value)} />
      <button onClick={upload}>click</button>

      <div>{filePath}</div>
    </div>
  );
}

export default App;
