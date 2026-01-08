import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export namespace ObjectUtils {
  export function updateObjectDynamically<T extends object>(
      obj: T,
      path: string,
      value: any
  ): T {
      const keys = path.split('.');
      const result = { ...obj };
      
      let current: any = result;
      for (let i = 0; i < keys.length - 1; i++) {
          const key = keys[i];
          current[key] = { ...current[key] };
          current = current[key];
      }
      
      current[keys[keys.length - 1]] = value;
      return result;
  }

  export function getFieldValue<T, K extends keyof T>(obj: T, key: K): T[K] {
      return obj[key];
  }
}
