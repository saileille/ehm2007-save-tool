// Date-related functions.

import { invoke } from "@tauri-apps/api/core";

export const getInGameDateText = async (): Promise<string> => {
    const dates: [string, string] = await invoke("get_ingame_date");
    if (dates[0] === dates[1]) {
        return `Estimated date is ${dates[0]}.`;
    }
    else {
        return `Estimated date is between ${dates[0]} and ${dates[1]}.`;
    }
}