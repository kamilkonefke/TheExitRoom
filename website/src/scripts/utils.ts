export function open_window(id: string) {
    let app = document.getElementById(id);
    if(app) {
        app.style.opacity = "1";
        app.style.pointerEvents = "all";
    }

    let tab = document.getElementById(`${id}-tab`);
    if(tab) {
        tab.style.opacity = "1";
        tab.style.pointerEvents = "all";
        tab.style.position = "unset";
    }
}

export function close_window(id: string) {
    let temp = document.getElementById(id);
    if(temp) {
        temp.style.opacity = "0";
        temp.style.pointerEvents = "none";
    }

    let tab = document.getElementById(`${id}-tab`);
    if(tab) {
        tab.style.opacity = "0";
        tab.style.pointerEvents = "none";
        tab.style.position = "absolute";
    }
}
