export interface Project {
  id: string;
  name: string;
  path: string;
  available: boolean;
}

export interface Environment {
  name: string;
  hostname: string;
  remote_user: string;
  deploy_path: string;
  branch?: string;
  stage?: string;
  keep_releases?: number;
}

export interface DeploymentStatus {
  type: "Running" | "Completed";
  deployment_id: string;
  started_at?: string;
  success?: boolean;
  exit_code?: number;
  duration_secs?: number;
}

export interface DeploymentOutput {
  deployment_id: string;
  line: string;
  stream: "Stdout" | "Stderr";
}

export interface Release {
  name: string;
  date?: string;
  is_current: boolean;
  target?: string;
}

export interface DeployCompleteEvent {
  deployment_id: string;
  success: boolean;
  exit_code: number;
}
