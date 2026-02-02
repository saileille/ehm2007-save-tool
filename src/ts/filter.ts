// Filtering scripts.

import { invoke } from "@tauri-apps/api/core";


// Create the filter elements.
export const createFilterLayer = async (main: HTMLElement, filtersButton: HTMLButtonElement) => {
    const filterCanvas = document.createElement("div");
    filterCanvas.id = "filter-canvas";
    filterCanvas.style.display = "none";

    const filterMenu = document.createElement("div");
    filterMenu.id = "filter-menu";
    filterMenu.style.display = "none";

    const nations: [number, string][] = await invoke("get_filter_data");
    const datalist = document.createElement("select");
    datalist.id = "nation";
    for (const nation of nations) {
        const option = document.createElement("option");
        option.value = nation[0].toString();
        option.textContent = nation[1];
        datalist.appendChild(option);
    }

    filterMenu.appendChild(datalist);

    filtersButton.onclick = () => {
        filterCanvas.style.display = "inherit";
        filterMenu.style.display = "inherit";
    };

    main.append(filterMenu, filterCanvas);
};