var grids = document.querySelector(".grids").getElementsByClassName("grid-container");
var navItems = document.querySelector(".navbar").getElementsByClassName("nav-item");
const { invoke } = window.__TAURI__.tauri;
const { emit, listen } = window.__TAURI__.event;
const dialog = window.__TAURI__.dialog;




// Navbar
function navGrids(event) {
    let navItems = document.querySelector(".navbar").getElementsByClassName("nav-item");
    let grids = document.querySelector(".grids").getElementsByClassName("grid-container");

    Array.from(grids).forEach(grid => {
        grid.classList.add("inactive");
        grid.classList.remove("active");
    })
    Array.from(navItems).forEach(item => {
        item.classList.add("inactive");
        item.classList.remove("active");
    })
    let index = Array.from(navItems).indexOf(event.srcElement);

    event.srcElement.classList.remove("inactive");
    event.srcElement.classList.add("active");

    grids[index].classList.remove("inactive");
    grids[index].classList.add("active");


} 

Array.from(navItems).forEach(item => {
    item.addEventListener("click", navGrids);

})

// Dot Nav
function navGridGroups(event) {
    let gridGroups = event.srcElement.parentElement.parentElement.getElementsByClassName("grid-group");
    let dots = event.srcElement.parentElement.getElementsByClassName("dot");

    Array.from(dots).forEach(dot => {
        dot.classList.remove("active");
        dot.classList.add("inactive");
    });
    Array.from(gridGroups).forEach(group => {
        group.classList.remove("active");
        group.classList.add("inactive");
    });
    event.srcElement.classList.remove("inactive");
    event.srcElement.classList.add("active");
    let index = Array.from(dots).indexOf(event.srcElement);
    gridGroups[index].classList.remove("inactive");
    gridGroups[index].classList.add("active");

}

Array.from(grids).forEach(grid => {
    let dots = grid.querySelector(".dot-nav")
    if (!dots) return;
    dots = dots.getElementsByClassName("dot"); 

    Array.from(dots).forEach(dot => {
        dot.addEventListener("click", navGridGroups)
    });
});


function goToTop() {
    window.scrollTo({'top':0, 'behavior':'smooth'})
}

document.querySelector(".up-arrow").addEventListener('click', goToTop);
