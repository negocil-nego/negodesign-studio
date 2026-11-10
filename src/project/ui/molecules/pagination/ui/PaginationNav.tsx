// @ts-nocheck
import { useState, useEffect } from "react";
import { StyleComponent } from "../../../../data/style.component";
import type { PaginationNavProps } from "../data/types";

export default function PaginationNav({
  totalPages = 5,
  currentPage = 1,
  size = "md",
  disabledFirst = false,
  onPageChange,
}: PaginationNavProps) {
  const [page, setPage] = useState(currentPage);

  useEffect(() => {
    setPage(currentPage);
  }, [currentPage]);

  const pages = Array.from({ length: totalPages }, (_, idx) => idx + 1);

  function goTo(next: number) {
    if (next < 1 || next > totalPages) return;
    setPage(next);
    onPageChange?.(next);
  }

  return (
    <nav aria-label="Pagination" className={StyleComponent.pagination.nav}>
      <ul className={`${StyleComponent.pagination.list} ${StyleComponent.pagination.size[size]}`}>
        <li className={StyleComponent.pagination.item}>
          <button
            type="button"
            disabled={disabledFirst || page <= 1}
            onClick={() => goTo(page - 1)}
            className={StyleComponent.pagination.link}
          >
            &laquo;
          </button>
        </li>
        {pages.map((num) => {
          const active = num === page;
          return (
            <li
              key={num}
              className={`${StyleComponent.pagination.item} ${
                active ? StyleComponent.pagination.itemActive : ""
              }`}
            >
              <button
                type="button"
                onClick={() => goTo(num)}
                className={`${StyleComponent.pagination.link} ${
                  active ? StyleComponent.pagination.linkActive : ""
                }`}
              >
                {num}
              </button>
            </li>
          );
        })}
        <li className={StyleComponent.pagination.item}>
          <button
            type="button"
            disabled={page >= totalPages}
            onClick={() => goTo(page + 1)}
            className={StyleComponent.pagination.link}
          >
            &raquo;
          </button>
        </li>
      </ul>
    </nav>
  );
}
