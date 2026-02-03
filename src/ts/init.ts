import { invoke } from "@tauri-apps/api/core";
import { initialisePaging, PAGE, ROWS_PER_PAGE } from "./paging";
import { daysToDateString, getInGameDateText } from "./date";
import { createFilterLayer } from "./filter";

type Player = {
    forename: string,
    surname: string,
    positions: number[],
    columns: [string | number],
};

export let PLAYERS: Player[] = [];
const HEADERS = [
    "Name",
    "Staff ID",
    "Random",
    "Nation",
    "Second Nation",
    "Birthday",
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
export const fetchPlayers = async (nationId: number,
    nationalTeamCheck: boolean,
    countryChoiceCheck: boolean,
    earliestBirthYear: number,
    excludeNHL: boolean,
    excludeNA: boolean) => {
    invoke("fetch_players", {
        "headers": HEADERS,
        "nationId": nationId,
        "nationalTeamCheck": nationalTeamCheck,
        "countryChoiceCheck": countryChoiceCheck,
        "earliestBirthYear": earliestBirthYear,
        "excludeNhl": excludeNHL,
        "excludeNa": excludeNA,
    }).then((players) => {
        PLAYERS = players as Player[];
        initialisePaging();
        overwriteTable();
    });
};

// Replace the save-loading start page with the player table.
const createPlayerView = async () => {
    const inGameDate = document.createElement("span");
    inGameDate.textContent = await getInGameDateText();

    const main = document.getElementsByTagName("main")[0];
    main.innerHTML = "";

    const loadSaveButton = document.createElement("button");
    loadSaveButton.textContent = "Load Save";
    loadSaveButton.onclick = loadSave;
    loadSaveButton.disabled = true; // Because replacing that info is a pain.

    const filtersButton = document.createElement("button");
    filtersButton.textContent = "Filters";

    const prevButton = document.createElement("button");
    prevButton.textContent = "Previous Page";
    prevButton.id = "prev-page";
    prevButton.disabled = true;

    const pageNumbers = document.createElement("span");
    pageNumbers.id = "page-numbers";

    const nextButton = document.createElement("button");
    nextButton.textContent = "Next Page";
    nextButton.id = "next-page";
    nextButton.disabled = true;

    const table = document.createElement("table");
    const thead = document.createElement("thead");

    const tbody = document.createElement("tbody");
    tbody.id = "players";

    const tr = document.createElement("tr");
    tr.id = "headers";

    for (const header of HEADERS) {
        const th = document.createElement("th");
        th.textContent = header;

        tr.appendChild(th);
    }

    thead.appendChild(tr);
    table.append(thead, tbody);

    main.append(loadSaveButton, inGameDate, filtersButton, prevButton, pageNumbers, nextButton, table);
    await createFilterLayer(main, filtersButton);

    createSortingScripts();
};

// Get the value as a displayable one.
const getDisplayValue = (index: number, value: string | number): string => {
    const headerName = HEADERS[index];

    if (headerName === "Birthday") {
        return daysToDateString(value as number);
    }

    if (headerName === "GK Rating"
    || headerName === "LD Rating"
    || headerName === "RD Rating"
    || headerName === "LW Rating"
    || headerName === "C Rating"
    || headerName === "RW Rating") {
        let num = value as number;
        return `${(num * 100).toFixed(2)} %`;
    }

    return value.toString();
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
}

const createSortingScripts = () => {
    const tr = document.getElementById("headers") as HTMLTableRowElement;

    for (let i = 0; i < tr.children.length; i++) {
        const child = tr.children[i] as HTMLTableCellElement;
        child.onclick = () => {
            sortTable(i);
        };
    }
};

// Sort the table.
const sortTable = (n: number) => {
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
        else {
            sortGeneric(sortAscending, n);
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

// The generic sorting.
const sortGeneric = (sortAscending: number, n: number) => {
    PLAYERS.sort((a, b) => {
        if (a.columns[n] < b.columns[n]) {
            return -1 * sortAscending;
        }

        if (b.columns[n] < a.columns[n]) {
            return 1 * sortAscending;
        }

        return 0;
    });
}

const createCell = (content: string): HTMLTableCellElement => {
    const td = document.createElement("td");
    td.textContent = content;

    return td;
};

// Load a save.
const loadSave = async () => {
    let success = await invoke("load_save");
    if (!success) { return; }
    await createPlayerView();
    await fetchPlayers(-2, false, false, 0, false, false);
};

// Add the onclick event for the Load Save button here.
const enableLoadButton = () => {
    const button = document.getElementsByTagName("button")[0];
    button.onclick = loadSave;
};

enableLoadButton();