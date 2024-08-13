import { Task } from "../interfaces/task";
import { del, get, post, put } from "./util";

const TaskBaseUrl = "http://localhost:8080/tasks";

export const getTasks = async (): Promise<Task[]> => {
  return await get(TaskBaseUrl);
};

export const updateTask = async (data: Task): Promise<Task> => {
  return await put(TaskBaseUrl, data);
};

export const createTask = async (data: Task): Promise<Task> => {
  return await post(TaskBaseUrl, data);
};

export const deleteTask = async (data: Task): Promise<Task> => {
  return await del(TaskBaseUrl, data);
};
