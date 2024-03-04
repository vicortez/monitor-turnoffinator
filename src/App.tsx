import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";
import "./App.css";

function App() {
  const [cmdResponse, setCmdResponse] = useState<any>(null);
  const [delay, setDelay] = useState<number | null>(3600);

  async function turnOffMonitor(delaySecs: number) {
    console.log("vai rodar");

    const res: string = await invoke("turn_off_monitor", { delaySecs });
    console.log(res);
    setCmdResponse(res);
  }

  return (
    <div className="container">
      <p>Choose delay (in seconds) to turn off monitor</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          turnOffMonitor(delay || 0);
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setDelay(Number(e.currentTarget.value || null))}
          placeholder="Enter a delay (sec) ..."
          value={delay || ""}
        />
        <button type="submit">Turn off monitor</button>
      </form>

      {cmdResponse && (
        <>
          <p>Status: {cmdResponse.status}</p>
          {cmdResponse.stderr && <p>Error: {cmdResponse.stderr}</p>}
          {cmdResponse.stdout && <p>Result: {cmdResponse.stdout}</p>}
        </>
      )}
    </div>
  );
}

export default App;
