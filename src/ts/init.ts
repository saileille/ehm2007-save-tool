import { invoke } from "@tauri-apps/api/core";
import { initialisePaging, PAGE, ROWS_PER_PAGE } from "./paging";

type Player = [string | number];

export let PLAYERS: Player[] = [];

// Get the players from the database.
const fetchPlayers = async () => {
    invoke("fetch_players").then((players) => {
        PLAYERS = players as Player[];
        createSortingScripts();
        overwriteTable();

        initialisePaging();
    });
};

// Overwrite existing data in the player table without removing elements.
export const overwriteTable = () => {
    const tbody = document.getElementById("players") as HTMLTableSectionElement;
    let counter = 0;
    let i = PAGE * ROWS_PER_PAGE;

    console.log(i);

    while (counter < ROWS_PER_PAGE) {
        const player = PLAYERS[i]
        let tr: HTMLTableRowElement;

        // Use an existing row if one exists.
        if (tbody.children.length > counter) {
            tr = tbody.children[counter] as HTMLTableRowElement;
            for (const [i2, value] of player.entries()) {
                const td = tr.children[i2] as HTMLTableCellElement;
                td.textContent = value.toString();
            }
        }

        // Create a new one if needed.
        else {
            tr = document.createElement("tr");
            for (const value of player) {
                tr.appendChild(createCell(value));
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

    do {
        PLAYERS.sort((a, b) => {
            if (a[n] < b[n]) {
                return -1 * sortAscending;
            }
            else if (b[n] < a[n]) {
                return 1 * sortAscending;
            }
            else {
                return 0;
            }
        });

        // Check if anything changed.
        if (before !== JSON.stringify(PLAYERS)) {
            overwriteTable();
            break;
        }

        sortAscending *= -1;
    } while (sortAscending !== 1);
}

const createCell = (content: string | number): HTMLTableCellElement => {
    const td = document.createElement("td");
    td.textContent = content.toString();
    return td;
};

fetchPlayers();