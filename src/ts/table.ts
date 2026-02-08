import { invoke } from "@tauri-apps/api/core";
import { initialisePaging, PAGE, ROWS_PER_PAGE } from "./paging";

type Player = {
    forename: string,
    surname: string,
    dateOfBirth: [number, number, number],
    positions: number[],
    columns: [string | number],
};

export let PLAYERS: Player[] = [];
export const HEADERS = [
    "Name",
    "Random",
    "Nation",
    "Second Nation",
    "Age",
    "Birthday",
    "Birth Place",
    "Position",
    "GK Rating",
    "LD Rating",
    "RD Rating",
    "LW Rating",
    "C Rating",
    "RW Rating",
    "Club Contracted",
    "Club Playing",
    "Adaptability",
    "Ambition",
    "Determination",
    "Loyalty",
    "Pressure",
    "Professionalism",
    "Sportsmanship",
    "Temperament",
    "Current Ability",
    "Potential Ability",
    "Acceleration",
    "Aggression",
    "Agility",
    "Anticipation",
    "Balance",
    "Bravery",
    "Consistency",
    "Decisions",
    "Dirtiness",
    "Flair",
    "Important Matches",
    "Injury Proneness",
    "Influence",
    "Off The Puck",
    "Natural Fitness",
    "One On Ones",
    "Speed",
    "Passing",
    "Positioning",
    "Reflexes",
    "Stamina",
    "Strength",
    "Teamwork",
    "Versatility",
    "Creativity",
    "Work Rate",
    "GK",
    "LD",
    "RD",
    "LW",
    "C",
    "RW",
    "Agitation",
    "Blocker",
    "Checking",
    "Defensive Role",
    "Deflections",
    "Deking",
    "Faceoffs",
    "Fighting",
    "Glove",
    "Hitting",
    "Offensive Role",
    "Pass Tendency",
    "Pokecheck",
    "Rebound Control",
    "Recovery",
    "Slapshot",
    "Stickhandling",
    "Wristshot",
];

// Get the players from the database.
export const fetchPlayers = async (
    birthYears: [number, number],
    nationalTeamCheck: boolean | undefined,
    secondNationalityCheck: boolean | undefined,
    declaredCheck: boolean | undefined,
    includeNationalities: number[],
    excludeNationalities: number[],
    includeClubsContracted: number[],
    excludeClubsContracted: number[],
    includeClubsPlaying: number[],
    excludeClubsPlaying: number[],
    includeCompsContracted: number[],
    excludeCompsContracted: number[],
    includeCompsPlaying: number[],
    excludeCompsPlaying: number[],
    includeNationsContracted: number[],
    excludeNationsContracted: number[],
    includeNationsPlaying: number[],
    excludeNationsPlaying: number[],
) => {
    invoke("fetch_players", {
        "headers": HEADERS,
        "birthYears": birthYears,
        "nationalTeamCheck": nationalTeamCheck,
        "secondNationalityCheck": secondNationalityCheck,
        "declaredCheck": declaredCheck,
        "includeNationalities": includeNationalities,
        "excludeNationalities": excludeNationalities,
        "includeClubsContracted": includeClubsContracted,
        "excludeClubsContracted": excludeClubsContracted,
        "includeClubsPlaying": includeClubsPlaying,
        "excludeClubsPlaying": excludeClubsPlaying,
        "includeCompsContracted": includeCompsContracted,
        "excludeCompsContracted": excludeCompsContracted,
        "includeCompsPlaying": includeCompsPlaying,
        "excludeCompsPlaying": excludeCompsPlaying,
        "includeNationsContracted": includeNationsContracted,
        "excludeNationsContracted": excludeNationsContracted,
        "includeNationsPlaying": includeNationsPlaying,
        "excludeNationsPlaying": excludeNationsPlaying,
    }).then((players) => {
        PLAYERS = players as Player[];
        initialisePaging();
        overwriteTable();
    });
};

// Overwrite existing data in the player table without removing elements.
export const overwriteTable = () => {
    const tbody = document.getElementById("players") as HTMLTableSectionElement;
    let counter = 0;
    let i = PAGE * ROWS_PER_PAGE;

    while (counter < ROWS_PER_PAGE && i < PLAYERS.length) {
        const player = PLAYERS[i]
        let tr: HTMLTableRowElement;

        // Use an existing row if one exists.
        if (tbody.children.length > counter) {
            tr = tbody.children[counter] as HTMLTableRowElement;
            for (const [i2, value] of player.columns.entries()) {
                const td = tr.children[i2] as HTMLTableCellElement;
                td.textContent = getDisplayValue(i2, value);
            }
        }

        // Create a new one if needed.
        else {
            tr = document.createElement("tr");
            for (const [i2, value] of player.columns.entries()) {
                tr.appendChild(createCell(getDisplayValue(i2, value)));
            }

            tbody.appendChild(tr);
        }

        counter++;
        i++;
    }

    // Remove possible unused rows.
    while (counter < tbody.children.length) {
        tbody.removeChild(tbody.lastChild as Node);
    }
};

// Get the value as a displayable one.
const getDisplayValue = (index: number, value: string | number): string => {
    const headerName = HEADERS[index];

    if (headerName === "GK Rating"
    || headerName === "LD Rating"
    || headerName === "RD Rating"
    || headerName === "LW Rating"
    || headerName === "C Rating"
    || headerName === "RW Rating") {
        let num = value as number;
        if (num === -1.0) {
            return "";
        }

        return `${(num * 100).toFixed(2)}`;
    }

    return value.toString();
};

// Sort the table.
export const sortTable = (n: number) => {
    let sortAscending = 1;
    const before = JSON.stringify(PLAYERS);

    const columnName = (document.getElementById("headers") as HTMLTableRowElement).children[n].textContent;

    do {
        if (columnName === "Name") {
            sortName(sortAscending);
        }

        else if (columnName === "Position") {
            sortPosition(sortAscending);
        }

        else if (columnName === "Birthday" || columnName === "Age") {
            sortBirthday(sortAscending);
        }

        else if (
            columnName === "GK Rating" ||
            columnName === "LD Rating" ||
            columnName === "RD Rating" ||
            columnName === "LW Rating" ||
            columnName === "C Rating" ||
            columnName === "RW Rating"
        ) {
            sortGeneric(sortAscending, n, -1.0);
        }

        else {
            sortGeneric(sortAscending, n, "");
        }

        // Check if anything changed.
        if (before !== JSON.stringify(PLAYERS)) {
            overwriteTable();
            break;
        }

        sortAscending *= -1;
    } while (sortAscending !== 1);
};

// Sort by the player name.
const sortName = (sortAscending: number) => {
    PLAYERS.sort((a, b) => {
        if (a.surname < b.surname) {
            return -1 * sortAscending;
        }

        if (b.surname < a.surname) {
            return 1 * sortAscending;
        }

        if (a.forename < b.forename) {
            return -1 * sortAscending;
        }

        if (b.forename < a.forename) {
            return 1 * sortAscending;
        }

        return 0;
    });
};

// Sort by the player position.
const sortPosition = (sortAscending: number) => {
    PLAYERS.sort((a, b) => {
        for (let i = 0; i < 6; i++) {
            const a_pos = a.positions[i];
            const b_pos = b.positions[i];

            if (a_pos === undefined && b_pos === undefined) {
                break;
            }

            if (a_pos === b_pos) {
                continue;
            }

            if (a_pos === undefined && b_pos !== undefined) {
                return -1 * sortAscending;
            }

            if (b_pos === undefined && a_pos !== undefined) {
                return 1 * sortAscending;
            }

            return (a_pos - b_pos) * sortAscending;
        }

        return 0;
    });
};

const sortBirthday = (sortAscending: number) => {
    PLAYERS.sort((a, b) => {
        let diff = a.dateOfBirth[0] - b.dateOfBirth[0];
        if (diff !== 0) {
            return diff * sortAscending;
        }

        diff = a.dateOfBirth[1] - b.dateOfBirth[1];
        if (diff !== 0) {
            return diff * sortAscending;
        }

        return (a.dateOfBirth[2] - b.dateOfBirth[2]) * sortAscending;
    });
};

// The generic sorting.
const sortGeneric = (sortAscending: number, n: number, emptyColumn: string | number) => {
    PLAYERS.sort((a, b) => {
        const aCol = a.columns[n];
        const bCol = b.columns[n];

        if (aCol === bCol) {
            return 0;
        }

        // Make the empty columns always show up last.
        if (aCol === emptyColumn) {
            return 1;
        }

        if (bCol === emptyColumn) {
            return -1;
        }

        if (aCol < bCol) {
            return -1 * sortAscending;
        }

        // if (bCol < aCol) {
            return 1 * sortAscending;
        // }
    });
}

const createCell = (content: string): HTMLTableCellElement => {
    const td = document.createElement("td");
    td.textContent = content;

    return td;
};