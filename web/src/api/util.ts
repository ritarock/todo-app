import { Task } from "../interfaces/task";

export const get = async (path: string): Promise<Task[]> => {
  return fetch(path, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
    },
  }).then<Task[]>((res) => res.json());
};

export const post = async (path: string, data: Task): Promise<Task> => {
  return await fetch(path, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  }).then<Task>((res) => res.json());
};

export const put = async (path: string, data: Task): Promise<Task> => {
  return await fetch(path + "/" + data.id, {
    method: "PUT",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  }).then<Task>((res) => res.json());
};

export const del = async (path: string, data: Task): Promise<Task> => {
  return await fetch(path + "/" + data.id, {
    method: "DELETE",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  }).then<Task>((res) => res.json());
};
