// Date-related functions.

import { invoke } from "@tauri-apps/api/core";

const MONTHS = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

// Convert the single number of days since 1900-02-01 to a date.
const daysToDate = (days: number): [number, number, number] => {
    // Start from 1900-01-01 to make things easier.
    days += 31;

    const yearInfo = getYear(days);
    const year = yearInfo[0];
    days = yearInfo[1];

    const monthInfo = getMonth(days, isLeapYear(year));
    return [year, monthInfo[0], monthInfo[1]];
};

export const daysToDateString = (days: number): string => {
    const date = daysToDate(days);
    return `${date[2]}.${date[1]}.${date[0]}`;
}

// Get the year of the date. Return it and days remaining.
const getYear = (days: number): [number, number] => {
    let year = 1900;

    while (true) {
        const daysInYear = isLeapYear(year) ? 366 : 365;
        if (days >= daysInYear) {
            year++;
            days -= daysInYear;
        }
        else {
            break;
        }
    }

    return [year, days];
};

// Get the month of the date. Return it and days remaining.
const getMonth = (days: number, leapYear: boolean): [number, number] => {
    let month = 13;
    for (const [i, monthDays] of MONTHS.entries()) {
        let actualDays = (i === 1 && leapYear) ? (monthDays + 1) : monthDays

        if (days >= actualDays) {
            days -= actualDays;
        }
        else {
            month = i + 1;
            break;
        }
    }

    return [month, days + 1];
};

// Check if the year is a leap year.
const isLeapYear = (year: number): boolean => {
    if (year % 4 === 0) {
        if (year % 100 === 0 && year % 400 !== 0) {
            return false;
        }
        return true;
    }
    return false;
}

export const getInGameDateText = async (): Promise<string> => {
    const dates: [number, number] = await invoke("get_ingame_date");
    if (dates[0] === dates[1]) {
        return `Estimated date is ${daysToDateString(dates[0])}.`;
    }
    else {
        return `Estimated date is between ${daysToDateString(dates[0])} and ${daysToDateString(dates[1])}.`;
    }
}