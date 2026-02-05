// Paging-related code.
import { overwriteTable, PLAYERS } from "./table";

export let PAGE = 0;
export const ROWS_PER_PAGE = 50;

const getPageCount = (): number => {
    return Math.ceil(PLAYERS.length / ROWS_PER_PAGE);
};

const updatePagingText = () => {
    const span = document.getElementById("page-numbers") as HTMLSpanElement;
    span.textContent = `${PAGE + 1} / ${getPageCount()}`;
};

const toggleNextPageButton = () => {
    const button = document.getElementById("next-page") as HTMLButtonElement;
    const pageCount = getPageCount();
    if (PAGE + 1 === pageCount || pageCount === 0) {
        button.disabled = true;
    }
    else {
        button.disabled = false;
    }
};

const togglePrevPageButton = () => {
    const button = document.getElementById("prev-page") as HTMLButtonElement;
    if (PAGE === 0) {
        button.disabled = true;
    }
    else {
        button.disabled = false;
    }
};

const toNextPage = () => {
    PAGE++;
    overwriteTable();

    const button = document.getElementById("prev-page") as HTMLButtonElement;
    button.disabled = false;
    toggleNextPageButton();
    updatePagingText();
};

const toPrevPage = () => {
    PAGE--;
    overwriteTable();

    const button = document.getElementById("next-page") as HTMLButtonElement;
    button.disabled = false;
    togglePrevPageButton();
    updatePagingText();
};

export const initialisePaging = () => {
    PAGE = 0;
    updatePagingText();
    toggleNextPageButton();
    togglePrevPageButton();

    const nextButton = document.getElementById("next-page") as HTMLButtonElement;
    nextButton.onclick = toNextPage;

    const prevButton = document.getElementById("prev-page") as HTMLButtonElement;
    prevButton.onclick = toPrevPage;
};