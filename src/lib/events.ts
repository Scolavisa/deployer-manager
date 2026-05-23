import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { DeploymentOutput, DeployCompleteEvent } from "../types";

export async function onDeployOutput(
  callback: (output: DeploymentOutput) => void
): Promise<UnlistenFn> {
  return listen<DeploymentOutput>("deploy_output", (event) => {
    callback(event.payload);
  });
}

export async function onDeployComplete(
  callback: (event: DeployCompleteEvent) => void
): Promise<UnlistenFn> {
  return listen<DeployCompleteEvent>("deploy_complete", (event) => {
    callback(event.payload);
  });
}
