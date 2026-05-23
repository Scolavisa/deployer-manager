import { writable } from "svelte/store";
import type { Project, DeploymentOutput, DeploymentStatus } from "../types";

export const projects = writable<Project[]>([]);
export const selectedProjectId = writable<string | null>(null);
export const deploymentOutputs = writable<Map<string, DeploymentOutput[]>>(new Map());
export const deploymentStatuses = writable<Map<string, DeploymentStatus>>(new Map());
