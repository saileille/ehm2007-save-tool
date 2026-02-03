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
    createCanPlayForCountryCheck(filterMenu);
    createCanChooseCountryCheck(filterMenu);
    createBirthYearFilter(filterMenu);
    createNHLExclusion(filterMenu);
    createNorthAmericaExclusion(filterMenu);

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

// Create the checkbox for national team eligibility.
const createCanPlayForCountryCheck = (filterMenu: HTMLDivElement) => {
    const label = document.createElement("label");
    label.htmlFor = "can-play-for-country";
    label.textContent = "Can Play for the National Team";

    const checkbox = document.createElement("input");
    checkbox.type = "checkbox";
    checkbox.id = "can-play-for-country";

    filterMenu.append(checkbox, label);
};

// Create the checkbox for filtering players who can play for two countries.
const createCanChooseCountryCheck = (filterMenu: HTMLDivElement) => {
    const label = document.createElement("label");
    label.htmlFor = "can-choose-country";
    label.textContent = "Can Choose the National Team";

    const checkbox = document.createElement("input");
    checkbox.type = "checkbox";
    checkbox.id = "can-choose-country";

    filterMenu.append(checkbox, label);
};

// Create a filter for birth year.
const createBirthYearFilter = (filterMenu: HTMLDivElement) => {
    const label = document.createElement("label");
    label.htmlFor = "earliest-birth-year";
    label.textContent = "Earliest Birth Year";

    const input = document.createElement("input");
    input.type = "number";
    input.id = "earliest-birth-year";
    input.value = "0";

    filterMenu.append(label, input);
};

// Create a filter for excluding NHL players.
const createNHLExclusion = (filterMenu: HTMLDivElement) => {
    const label = document.createElement("label");
    label.htmlFor = "exclude-nhl";
    label.textContent = "Ignore NHL Players";

    const checkbox = document.createElement("input");
    checkbox.type = "checkbox";
    checkbox.id = "exclude-nhl";

    filterMenu.append(checkbox, label);
};

// Create a filter for excluding players playing in North America.
const createNorthAmericaExclusion = (filterMenu: HTMLDivElement) => {
    const label = document.createElement("label");
    label.htmlFor = "exclude-na";
    label.textContent = "Ignore Players in North America";

    const checkbox = document.createElement("input");
    checkbox.type = "checkbox";
    checkbox.id = "exclude-na";

    filterMenu.append(checkbox, label);
};

// Apply the filters.
const applyFilters = async () => {
    const nationId = Number((document.getElementById("nation") as HTMLSelectElement).value);
    const nationalTeamCheck = (document.getElementById("can-play-for-country") as HTMLInputElement).checked;
    const countryChoiceCheck = (document.getElementById("can-choose-country") as HTMLInputElement).checked;
    const earliestBirthYear = Number((document.getElementById("earliest-birth-year") as HTMLInputElement).value);
    const excludeNHL = (document.getElementById("exclude-nhl") as HTMLInputElement).checked;
    const excludeNA = (document.getElementById("exclude-na") as HTMLInputElement).checked;
    await fetchPlayers(nationId, nationalTeamCheck, countryChoiceCheck, earliestBirthYear, excludeNHL, excludeNA);
}