// Filtering scripts.

import { invoke } from "@tauri-apps/api/core";
import { fetchPlayers } from "./table";

type IncludeExclude = "Include" | "Exclude";

// Create the filter elements.
export const createFilterLayer = async (main: HTMLElement, filtersButton: HTMLButtonElement) => {
    const filterEffect = document.createElement("div");
    filterEffect.id = "filter-canvas";
    filterEffect.style.display = "none";

    const filterMenu = document.createElement("div");
    filterMenu.id = "filter-menu";
    filterMenu.style.display = "none";

    const filterContainer = document.createElement("div");
    filterContainer.id = "filter-container";
    filterContainer.className = "filter-category";

    filtersButton.onclick = () => {
        filterEffect.style.display = "";
        filterMenu.style.display = "";

        // Remove the main scrollbar.
        document.body.style.overflow = "hidden";

        updateFilterColumns();
    };

    const filters = document.createElement("fieldset");
    filters.className = "filter-category";

    const legend = document.createElement("legend");
    legend.textContent = "Filters";
    filters.appendChild(legend);
    filterContainer.appendChild(filters);

    createBirthYearFilter(filters);
    createEitherYesNo(filters, "Can Play for Country", "can-play-for-country");
    createEitherYesNo(filters, "Has Second Nationality", "second-nationality");
    createEitherYesNo(filters, "Has Declared for Nation", "has-declared");

    createIncludeExcludeFieldset("Include", filterContainer);
    createIncludeExcludeFieldset("Exclude", filterContainer);

    const applyFiltersButton = document.createElement("button");
    applyFiltersButton.textContent = "Apply";
    applyFiltersButton.onclick = onApplyFiltersButtonClick;

    // Bind filter-apply to the enter key.
    onkeyup = (e) => {
        if (e.code === "Enter" && filterMenu.style.display !== "none") {
            onApplyFiltersButtonClick();
        }
    };

    filterMenu.append(filterContainer, applyFiltersButton);
    main.append(filterMenu, filterEffect);
};

// Create a section for search terms to be included.
const createIncludeExcludeFieldset = (type: IncludeExclude, filterContainer: HTMLDivElement) => {
    const div = document.createElement("div");

    const fieldset = document.createElement("fieldset");
    fieldset.className = "filter-category";
    const legend = document.createElement("legend");
    legend.textContent = type;

    const menu = document.createElement("select");

    const options = [
        ["default", "Add..."],
        ["nationality", "Nationality"],
        ["club-contracted", "Club Contracted"],
        ["club-playing", "Club Playing"],
        ["comp-contracted", "Competition Contracted"],
        ["comp-playing", "Competition Playing"],
        ["nation-contracted", "Nation Contracted"],
        ["nation-playing", "Nation Playing"],
    ];

    for (const optionData of options) {
        const option = document.createElement("option");
        option.value = optionData[0];
        option.textContent = optionData[1];
        menu.appendChild(option);
    }

    menu.onchange = () => {
        addCriterium(type, menu, fieldset);
        menu.value = "default";

        // Give the backend time before recalculating columns.
        setTimeout(updateFilterColumns, 50);
    };

    fieldset.appendChild(legend);
    div.append(menu, fieldset);

    filterContainer.appendChild(div);
};

// Add either include or exclude element.
const addCriterium = async (type: IncludeExclude, menu: HTMLSelectElement, container: HTMLFieldSetElement) => {
    const div = document.createElement("div");
    div.className = "criterium-container";

    const criterium = menu.value;
    const name = menu.selectedOptions[0].textContent;

    const typeLowerCase = type.toLowerCase();

    const label = document.createElement("label");
    label.textContent = name;

    const filter = document.createElement("input");
    filter.type = "text";

    const delButton = document.createElement("button");
    delButton.textContent = "Delete";

    let select: HTMLSelectElement | undefined;
    switch (criterium) {
        case "nationality":
            select = await createSelect("get_nations");
            break;

        case "club-contracted":
            select = await createSelect("get_clubs");
            break;

        case "club-playing":
            select = await createSelect("get_clubs");
            break;

        case "comp-contracted":
            select = await createSelect("get_comps");
            break;

        case "comp-playing":
            select = await createSelect("get_comps");
            break;

        case "nation-contracted":
            select = await createSelect("get_nations");
            break;

        case "nation-playing":
            select = await createSelect("get_nations");
            break;
    }

    if (select === undefined) {
        return;
    }

    select.className = `${typeLowerCase}-${criterium}`;

    filter.oninput = () => {
        searchFilter(filter, select);
    };

    delButton.onclick = () => {
        div.remove();
        updateFilterColumns();
    };

    div.append(label, select, filter, delButton);
    container.appendChild(div);
    filter.select();
};

// The function for a search filter.
const searchFilter = (filter: HTMLInputElement, select: HTMLSelectElement) => {
    const text = filter.value.toLowerCase();
    let chosen = false;
    for (const o of select.children) {
        const option = o as HTMLOptionElement;
        if (option.textContent.toLowerCase().includes(text)) {
            option.style.display = "";
            if (!chosen) {
                select.value = option.value;
                chosen = true;
            }
        }
        else {
            option.style.display = "none";
        }
    }
};

const createSelect = async (fn: string): Promise<HTMLSelectElement> => {
    const select = document.createElement("select");
    invoke(fn).then((d) => {
        const data = d as [number, string][];
        for (const item of data) {
            const option = document.createElement("option");
            option.value = item[0].toString();
            option.textContent = item[1];
            select.appendChild(option);
        }
    });

    return select;
};

// Create a radio group with three options.
const createEitherYesNo = (filterMenu: HTMLFieldSetElement, title: string, name: string) => {
    const div = document.createElement("div");
    const text = document.createTextNode(title);

    const labelEither = document.createElement("label");
    labelEither.htmlFor = `${name}-either`;
    labelEither.textContent = "Either";
    const either = document.createElement("input");
    either.type = "radio";
    either.id = `${name}-either`;
    either.value = `${name}-either`;
    either.name = name;
    either.checked = true;

    const labelYes = document.createElement("label");
    labelYes.htmlFor = `${name}-yes`;
    labelYes.textContent = "Yes";
    const yes = document.createElement("input");
    yes.type = "radio";
    yes.id = `${name}-yes`;
    yes.value = `${name}-yes`;
    yes.name = name;

    const labelNo = document.createElement("label");
    labelNo.htmlFor = `${name}-no`;
    labelNo.textContent = "No";
    const no = document.createElement("input");
    no.type = "radio"
    no.id = `${name}-no`;
    no.value = `${name}-no`;
    no.name = name;

    div.append(text, document.createElement("br"), either, labelEither, yes, labelYes, no, labelNo);
    filterMenu.appendChild(div);
};

// Create a filter for birth year.
const createBirthYearFilter = (filterMenu: HTMLFieldSetElement) => {
    const div = document.createElement("div");

    const label = document.createElement("label");
    label.textContent = "Birth Year";

    const inputEarliest = document.createElement("input");
    inputEarliest.type = "number";
    inputEarliest.id = "earliest-birth-year";
    inputEarliest.className = "year";
    inputEarliest.value = "0";

    const inputLatest = document.createElement("input");
    inputLatest.type = "number";
    inputLatest.id = "latest-birth-year";
    inputLatest.className = "year";
    inputLatest.value = "9999";

    div.append(label, inputEarliest, document.createTextNode(" - "), inputLatest);
    filterMenu.appendChild(div);
};

// Get undefined, true or false from a three-option radio group.
const getEitherYesNo = (name: string): boolean | undefined => {
    if ((document.getElementById(`${name}-yes`) as HTMLInputElement).checked) {
        return true;
    }

    if ((document.getElementById(`${name}-no`) as HTMLInputElement).checked) {
        return false;
    }

    return undefined;
};

// Apply the filters.
export const applyFilters = async () => {
    const birthYears: [number, number] = [
        Number((document.getElementById("earliest-birth-year") as HTMLInputElement).value),
        Number((document.getElementById("latest-birth-year") as HTMLInputElement).value),
    ];

    const nationalTeamCheck = getEitherYesNo("can-play-for-country");
    const secondNationalityCheck = getEitherYesNo("second-nationality");
    const declaredCheck = getEitherYesNo("has-declared");

    const includeNationalities = [];
    for (const element of document.getElementsByClassName("include-nationality")) {
        includeNationalities.push(Number((element as HTMLSelectElement).value));
    }

    const excludeNationalities = [];
    for (const element of document.getElementsByClassName("exclude-nationality")) {
        excludeNationalities.push(Number((element as HTMLSelectElement).value));
    }

    const includeClubsContracted = [];
    for (const element of document.getElementsByClassName("include-club-contracted")) {
        includeClubsContracted.push(Number((element as HTMLSelectElement).value));
    }

    const excludeClubsContracted = [];
    for (const element of document.getElementsByClassName("exclude-club-contracted")) {
        excludeClubsContracted.push(Number((element as HTMLSelectElement).value));
    }

    const includeClubsPlaying = [];
    for (const element of document.getElementsByClassName("include-club-playing")) {
        includeClubsPlaying.push(Number((element as HTMLSelectElement).value));
    }

    const excludeClubsPlaying = [];
    for (const element of document.getElementsByClassName("exclude-club-playing")) {
        excludeClubsPlaying.push(Number((element as HTMLSelectElement).value));
    }

    const includeCompsContracted = [];
    for (const element of document.getElementsByClassName("include-comp-contracted")) {
        includeCompsContracted.push(Number((element as HTMLSelectElement).value));
    }

    const excludeCompsContracted = [];
    for (const element of document.getElementsByClassName("exclude-comp-contracted")) {
        excludeCompsContracted.push(Number((element as HTMLSelectElement).value));
    }

    const includeCompsPlaying = [];
    for (const element of document.getElementsByClassName("include-comp-playing")) {
        includeCompsPlaying.push(Number((element as HTMLSelectElement).value));
    }

    const excludeCompsPlaying = [];
    for (const element of document.getElementsByClassName("exclude-comp-playing")) {
        excludeCompsPlaying.push(Number((element as HTMLSelectElement).value));
    }

    const includeNationsContracted = [];
    for (const element of document.getElementsByClassName("include-nation-contracted")) {
        includeNationsContracted.push(Number((element as HTMLSelectElement).value));
    }

    const excludeNationsContracted = [];
    for (const element of document.getElementsByClassName("exclude-nation-contracted")) {
        excludeNationsContracted.push(Number((element as HTMLSelectElement).value));
    }

    const includeNationsPlaying = [];
    for (const element of document.getElementsByClassName("include-nation-playing")) {
        includeNationsPlaying.push(Number((element as HTMLSelectElement).value));
    }

    const excludeNationsPlaying = [];
    for (const element of document.getElementsByClassName("exclude-nation-playing")) {
        excludeNationsPlaying.push(Number((element as HTMLSelectElement).value));
    }

    await fetchPlayers(
        birthYears,
        nationalTeamCheck,
        secondNationalityCheck,
        declaredCheck,
        includeNationalities,
        excludeNationalities,
        includeClubsContracted,
        excludeClubsContracted,
        includeClubsPlaying,
        excludeClubsPlaying,
        includeCompsContracted,
        excludeCompsContracted,
        includeCompsPlaying,
        excludeCompsPlaying,
        includeNationsContracted,
        excludeNationsContracted,
        includeNationsPlaying,
        excludeNationsPlaying,
    );

    // Re-enable the scrollbar.
    document.body.style.overflow = "";
}

const onApplyFiltersButtonClick = () => {
    applyFilters();
    const filterMenu = document.getElementById("filter-menu") as HTMLDivElement;
    const filterEffect = document.getElementById("filter-canvas") as HTMLDivElement;

    filterMenu.style.display = "none";
    filterEffect.style.display = "none";
};

// Update the filter widths.
const updateFilterColumns = () => {
    const filterContainer = document.getElementById("filter-container") as HTMLDivElement;
    filterContainer.style.gridTemplateColumns = "";

    let largestColumn = 0.0;
    for (const category of document.getElementsByClassName("filter-category")) {
        if (category.tagName === "DIV") {
            continue;
        }

        const cat = category as HTMLFieldSetElement;
        cat.style.gridTemplateColumns = "";
        const columnWidth = calculateColumnWidth(cat);
        if (columnWidth > largestColumn) {
            largestColumn = columnWidth;
        }
    }

    filterContainer.style.gridTemplateColumns = `repeat(auto-fit, minmax(${largestColumn + 30}px, 1fr))`;
};

// Calculate the minimum column width for a given grid.
const calculateColumnWidth = (grid: HTMLFieldSetElement): number => {
    let largestColumn = 0.0;
    for (const child of grid.children) {
        if (child.tagName === "LEGEND") {
            continue;
        }

        const width = child.getBoundingClientRect().width;
        if (width > largestColumn) {
            largestColumn = width;
        }
    }

    grid.style.gridTemplateColumns = `repeat(auto-fit, minmax(${largestColumn}px, 1fr))`;
    return largestColumn;
};