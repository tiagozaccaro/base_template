import { test, expect } from '@playwright/test';

test.describe('home page', () => {
	test('renders the hero and echo sections', async ({ page }) => {
		await page.goto('/');

		await expect(page.locator('#hero')).toBeVisible();
		await expect(page.locator('#header')).toBeVisible();
		await expect(page.locator('#echo')).toBeVisible();
		await expect(page.getByRole('heading', { name: 'ServerFn Echo' })).toBeVisible();
	});

	test('navigates to the blog page', async ({ page }) => {
		await page.goto('/');
		await page.getByRole('link', { name: 'Blog' }).click();

		await expect(page).toHaveURL(/\/blog\/1/);
	});
});
