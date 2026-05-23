import { invoke } from "@tauri-apps/api/core";
import type { Project, Environment, DeploymentStatus, Release } from "../types";

// Project management
export async function registerProject(path: string): Promise<Project> {
  return invoke<Project>("register_project", { path });
}

export async function removeProject(projectId: string): Promise<void> {
  return invoke<void>("remove_project", { projectId });
}

export async function listProjects(): Promise<Project[]> {
  return invoke<Project[]>("list_projects");
}

export async function getProject(projectId: string): Promise<Project> {
  return invoke<Project>("get_project", { projectId });
}

// Environment discovery
export async function getEnvironments(projectId: string): Promise<Environment[]> {
  return invoke<Environment[]>("get_environments", { projectId });
}

// Deployment
export async function startDeployment(
  projectId: string,
  environment: string,
  tag?: string,
  branch?: string
): Promise<string> {
  return invoke<string>("start_deployment", {
    projectId,
    environment,
    tag: tag || null,
    branch: branch || null,
  });
}

export async function getDeploymentStatus(deploymentId: string): Promise<DeploymentStatus> {
  return invoke<DeploymentStatus>("get_deployment_status", { deploymentId });
}

// Release history
export async function getReleases(projectId: string, environment: string): Promise<Release[]> {
  return invoke<Release[]>("get_releases", { projectId, environment });
}

// Git operations
export async function getTags(projectId: string): Promise<string[]> {
  return invoke<string[]>("get_tags", { projectId });
}

export async function getBranches(projectId: string): Promise<string[]> {
  return invoke<string[]>("get_branches", { projectId });
}
