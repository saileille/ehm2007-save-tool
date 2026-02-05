import { invoke } from "@tauri-apps/api/core";
import { getInGameDateText } from "./date";
import { applyFilters, createFilterLayer } from "./filter";
import { HEADERS, sortTable } from "./table";

// Replace the save-loading start page with the player table.
const createPlayerView = async () => {
    const inGameDate = document.createElement("span");
    inGameDate.textContent = await getInGameDateText();

    const main = document.getElementsByTagName("main")[0];
    main.innerHTML = "";

    const loadSaveButton = document.createElement("button");
    loadSaveButton.textContent = "Load Save";
    loadSaveButton.onclick = loadSave;

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

const createSortingScripts = () => {
    const tr = document.getElementById("headers") as HTMLTableRowElement;

    for (let i = 0; i < tr.children.length; i++) {
        const child = tr.children[i] as HTMLTableCellElement;
        child.onclick = () => {
            sortTable(i);
        };
    }
};

// Load a save.
const loadSave = async () => {
    let success = await invoke("load_save");
    if (!success) { return; }
    await createPlayerView();

    // Get the players according to the filters set.
    await applyFilters();
};

// Add the onclick event for the Load Save button here.
const enableLoadButton = () => {
    const button = document.getElementsByTagName("button")[0];
    button.onclick = loadSave;
};

enableLoadButton();