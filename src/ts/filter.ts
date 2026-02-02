// Filtering scripts.

import { invoke } from "@tauri-apps/api/core";
import { fetchPlayers } from "./init";

// Create the filter elements.
export const createFilterLayer = async (main: HTMLElement, filtersButton: HTMLButtonElement) => {
    const filterCanvas = document.createElement("div");
    filterCanvas.id = "filter-canvas";
    filterCanvas.style.display = "none";

    const filterMenu = document.createElement("div");
    filterMenu.id = "filter-menu";
    filterMenu.style.display = "none";

    filtersButton.onclick = () => {
        filterCanvas.style.display = "";
        filterMenu.style.display = "";
    };

    await createNationFilter(filterMenu);

    const applyFiltersButton = document.createElement("button");
    applyFiltersButton.textContent = "Apply";
    applyFiltersButton.onclick = () => {
        applyFilters();
        filterMenu.style.display = "none";
        filterCanvas.style.display = "none";
    };

    filterMenu.appendChild(applyFiltersButton);

    main.append(filterMenu, filterCanvas);
};

// Create the filter for nationality.
const createNationFilter = async (filterMenu: HTMLDivElement) => {
    const nations: [number, string][] = await invoke("get_filter_data");

    const label = document.createElement("label");
    label.htmlFor = "nation";
    label.textContent = "Nationality";

    const datalist = document.createElement("select");
    datalist.id = "nation";
    for (const nation of nations) {
        const option = document.createElement("option");
        option.value = nation[0].toString();
        option.textContent = nation[1];
        datalist.appendChild(option);
    }

    filterMenu.append(label, datalist);
};

// Apply the filters.
const applyFilters = async () => {
    const nationId = Number((document.getElementById("nation") as HTMLSelectElement).value);
    await fetchPlayers(nationId);

}