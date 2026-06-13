import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";

import { ThemeToggle } from "./theme-toggle";

describe("ThemeToggle", () => {
  it("alterna a classe .dark no documento ao clicar", async () => {
    render(<ThemeToggle />);

    const botao = screen.getByRole("button", { name: /tema/i });
    const antes = document.documentElement.classList.contains("dark");

    await userEvent.click(botao);

    expect(document.documentElement.classList.contains("dark")).toBe(!antes);
  });
});
