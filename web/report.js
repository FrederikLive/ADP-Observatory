(() => {
  "use strict";
  const search = document.getElementById("contract-search");
  const list = document.getElementById("contract-list");
  const empty = document.getElementById("contract-filter-empty");
  if (!search || !list || !empty) return;
  const items = Array.from(list.querySelectorAll(".filterable"));
  function applyFilter() {
    const query = search.value.trim().toLocaleLowerCase();
    let visible = 0;
    for (const item of items) {
      const matches = !query || (item.textContent || "").toLocaleLowerCase().includes(query);
      item.hidden = !matches;
      if (matches) visible++;
    }
    empty.hidden = visible > 0 || items.length === 0;
  }
  search.addEventListener("input", applyFilter);
})();
