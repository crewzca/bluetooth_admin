import { useState, useEffect } from "react";

import { invoke } from "@tauri-apps/api/core";
import "./App.css";

type DeviceInfo = {
  name: string;
  address: string;
};

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [devices, setDevices] = useState<DeviceInfo[]>([]);

  async function reload() {
    invoke<DeviceInfo[]>("import_device").then(setDevices).catch(console.error);
    setGreetMsg("");
  }

  async function removeAll(name: string) {
    let flg = true;
    if (await invoke('remove_all', { name })) {
      reload();
      if (flg) { setGreetMsg("一括削除に成功しました") }
    } else {
      flg = false;
    }
    if (!flg) { setGreetMsg("一括削除に失敗しました") }
  }

  async function handleClick(address: string) {
    let flg = true;
    if (await invoke('remove_device', { address })) {
      reload();
      if (flg) { setGreetMsg("デバイスの削除に成功しました") }
    } else {
      flg = false;
    }
    if (!flg) { setGreetMsg("デバイスの削除に失敗しました") }
  }

  useEffect(() => {
    reload();
  }, []);

  return (
    <main className="container">
      <h1>BLEデバイス一覧</h1>

      <button onClick={() => reload()}>再読み込み</button>

      <p>{greetMsg}</p>
      <table>
        {devices.map((device, i) => (
          <tr key={i}>
            <td className="device-name">{device.name}</td>
            <td><button onClick={() => handleClick(device.address)}>削除</button></td>
            <td><button onClick={() => removeAll(device.name)}>一括削除</button></td>
          </tr>
        ))}
      </table>
    </main >
  );
}

export default App;
