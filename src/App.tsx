import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [url, setUrl] = useState("");
  const [downloading, setDownloading] = useState(false);
  const [destination, setDestination] = useState("/Users/ricky/Downloads");
  const [isModifyingDestination, setIsModifyingDestination] = useState(false);
  const [currentPid, setCurrentPid] = useState(0);

  async function downloadSongs() {
    try {
      setDownloading(true);

      // if url is empty, return
      if (url === "") {
        return;
      }

      // if url has youtube in it, invoke command youtube_dl
      if (url.includes("youtube")) {
        const result = await invoke("youtube_dl", {
          url,
          destination,
        });

        setCurrentPid(Number(result));
      }

      // if url has soundcloud in it, invoke command soundcloud_dl
      if (url.includes("soundcloud")) {
        const result = await invoke("soundcloud_dl", { url, destination });

        setCurrentPid(Number(result));
      }
    } catch (error) {
      console.error(error);
    } finally {
      setDownloading(false);
    }
  }

  async function killProcess() {
    try {
      const result = await invoke("kill_process", { pid: String(currentPid) });
      console.log(result);
    } catch (error) {
      console.error(error);
    } finally {
      setCurrentPid(0);
    }
  }

  return (
    <div className="container">
      <div className="row">
        <img src="/128x128.png" alt="soundtube" width={128} height={128} />
      </div>
      <div className="row margin-bottom-xl">
        <div className="col">
          {isModifyingDestination && (
            <input
              onChange={(e) => setDestination(e.currentTarget.value)}
              onBlur={() => {
                setIsModifyingDestination(false);
              }}
              placeholder="New destination path ..."
            />
          )}
          <h4
            className="pointer"
            onClick={() => {
              setIsModifyingDestination(true);
            }}
          >
            {destination}
          </h4>
        </div>
      </div>
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          downloadSongs();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setUrl(e.currentTarget.value)}
          placeholder="URL goes here ..."
        />

        <button type="submit">Download</button>
      </form>
      {downloading && (
        <p>
          <div className="lds-ellipsis">
            <div></div>
            <div></div>
            <div></div>
            <div></div>
          </div>
        </p>
      )}
      <p>Current PID: {currentPid}</p>{" "}
      {currentPid !== 0 && <button onClick={(e) => killProcess()}>Kill</button>}
    </div>
  );
}

export default App;
