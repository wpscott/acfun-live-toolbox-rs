export * from "./qr";
export * from "./user";

export type Result<T> = Success<T> | Error;

export interface Success<T> {
  success: true;
  data: T;
}

export interface Error {
  success: false;
  data: null;
}
