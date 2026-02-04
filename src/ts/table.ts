import { daysToDateString } from "./date";
import { HEADERS, PLAYERS } from "./init";
import { PAGE, ROWS_PER_PAGE } from "./paging";


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