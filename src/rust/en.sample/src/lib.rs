#![no_std]
extern crate alloc;

use aidoku::{
	error::Result,
	prelude::*,
	std::{
		String, Vec,
	},
	Chapter, Filter, FilterType, Manga, MangaContentRating, MangaPageResult, MangaStatus, MangaViewer, Page
};
use alloc::string::ToString;

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, _: i32) -> Result<MangaPageResult> {

	let mut query = String::new();
	for filter in filters {
		match filter.kind {
			FilterType::Title => {
				query = filter.value.as_string()?.read();
			}
			_ => continue,
		}
	}

	let has_more = false;
	let mut mangas: Vec<Manga> = Vec::new();

	let titles = [
		"Wallpapers", "Nature", "3D Renders", "Travel", "Architecture & Interiors",
		"Textures & Patterns", "Street Photography", "Archival", "Experimental",
		"Animals", "Fashion & Beauty", "People", "Business & Work", "Food & Drink",
		"Health & Wellness", "Sports"
	];

	let covers = [
		"https://images.unsplash.com/photo-1717831380980-5b1572491d5f?ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&fm=jpg&q=90&w=300&fit=max",
		"https://images.unsplash.com/photo-1674382255247-1f901c0a6fcb?ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&fm=jpg&q=90&w=300&fit=max",
		"https://images.unsplash.com/photo-1722945629994-12ce3f1886ae?ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&fm=jpg&q=90&w=300&fit=max",
		"https://images.unsplash.com/photo-1720630941649-8e2dd3956990?ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&fm=jpg&q=90&w=300&fit=max",
		"https://images.unsplash.com/photo-1664016945875-9019cae26f24?ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&fm=jpg&q=90&w=300&fit=max",
		"https://images.unsplash.com/photo-1486718448742-163732cd1544?ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&fm=jpg&q=90&w=300&fit=max",
		"https://images.unsplash.com/photo-1726503454447-6c67b92dcc52?ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&fm=jpg&q=90&w=300&fit=max",
		"https://images.unsplash.com/photo-1580975016802-a10a00630106?ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&fm=jpg&q=90&w=300&fit=max",
		"https://images.unsplash.com/photo-1559731131-e8d78734c858?ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&fm=jpg&q=90&w=300&fit=max",
		"https://images.unsplash.com/photo-1704241260682-5bb0a120cdaa?ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&fm=jpg&q=90&w=300&fit=max",
		"https://images.unsplash.com/photo-1724313802311-da9f68e0875f?ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&fm=jpg&q=90&w=300&fit=max",
		"https://images.unsplash.com/photo-1724433530860-f094e39b64e7?ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&fm=jpg&q=90&w=300&fit=max",
		"https://images.unsplash.com/photo-1649194270591-8eead57b94c3?ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&fm=jpg&q=90&w=300&fit=max",
		"https://images.unsplash.com/photo-1525480122447-64809d765ec4?ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&fm=jpg&q=90&w=300&fit=max",
		"https://images.unsplash.com/photo-1633706961726-c67a0b691734?ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&fm=jpg&q=90&w=300&fit=max",
		"https://images.unsplash.com/photo-1604320239424-855007f7d759?ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&fm=jpg&q=90&w=300&fit=max"
	];

	if query.is_empty() {
		for i in 0..titles.len() {
			mangas.push(Manga {
				id: format!("id_{}", i + 1),
				cover: covers[i].to_string(),
				title: titles[i].to_string(),
				..Default::default()
			});
		}
	} else {
		// 将 query 转为小写，以进行不区分大小写的匹配
		let query_lower = query.to_lowercase();

		for i in 0..titles.len() {
			let title_lower = titles[i].to_lowercase();
			if title_lower.contains(&query_lower) {
				mangas.push(Manga {
					id: format!("id_{}", i + 1),
					cover: covers[i].to_string(),
					title: titles[i].to_string(),
					..Default::default()
				});
			}
		}
	}

	Ok(MangaPageResult {
		manga: mangas,
		has_more,
	})
}

#[get_manga_details]
fn get_manga_details(id: String) -> Result<Manga> {
	let mut cover = String::from("https://images.unsplash.com/photo-1604320239424-855007f7d759?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D");
	let mut title = String::from("Title");
	let author = String::from("Unsplash");
	let artist = String::new();
	let mut description = String::from("description");
	let url = String::from("https://github.com/CaretReader/CaretApp");
	const CATEGORIES: &[&str] = &["Sample", "Photos", "Royalty-free"];
	let categories: Vec<String> = CATEGORIES.iter().map(|&s| s.to_string()).collect();
	let status = MangaStatus::Unknown;
	let nsfw = MangaContentRating::Safe;
	let viewer = MangaViewer::Ltr;

	match id.as_str() {
		"id_1" => {
			cover = String::from("https://images.unsplash.com/photo-1717831380980-5b1572491d5f?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D");
			title = String::from("Wallpapers");
			description = String::from("From epic drone shots to inspiring moments in nature");
		},
		"id_2" => {
			cover = String::from("https://images.unsplash.com/photo-1674382255247-1f901c0a6fcb?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D");
			title = String::from("Nature");
			description = String::from("Photographers capture breathtaking landscapes, diverse flora & fauna, and mesmerizing natural phenomena, immersing you in the great outdoors.");
		},
		"id_3" => {
			cover = String::from("https://images.unsplash.com/photo-1722945629994-12ce3f1886ae?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D");
			title = String::from("3D Renders");
			description = String::from("Showcasing digitally rendered creations that blur reality and imagination, from architectural visualizations to fantastical worlds, highlighting digital craftsmanship.");
		},
		"id_4" => {
			cover = String::from("https://images.unsplash.com/photo-1720630941649-8e2dd3956990?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D");
			title = String::from("Travel");
			description = String::from("Photographers capture exploration and wanderlust through vibrant street scenes and cultural experiences, showing the beauty and diversity of global destinations.");
		},
		"id_5" => {
			cover = String::from("https://images.unsplash.com/photo-1664016945875-9019cae26f24?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D");
			title = String::from("Architecture & Interiors");
			description = String::from("Celebrating architectural marvels and interior designs, from sweeping cityscapes to intimate details, showcasing the beauty and functionality of built environments.");
		},
		"id_6" => {
			cover = String::from("https://images.unsplash.com/photo-1486718448742-163732cd1544?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D");
			title = String::from("Textures & Patterns");
			description = String::from("Whether you’re looking for stunning macro-photography or shots of complex architectural shapes — you’ve come to the right place.");
		},
		"id_7" => {
			cover = String::from("https://images.unsplash.com/photo-1726503454447-6c67b92dcc52?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D");
			title = String::from("Street Photography");
			description = String::from("From quiet passages in charming towns to the hustle and bustle of cities, this category examines street photography in every form.");
		},
		"id_8" => {
			cover = String::from("https://images.unsplash.com/photo-1580975016802-a10a00630106?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D");
			title = String::from("Archival");
			description = String::from("Exploring captivating art and photography from galleries, museums, and cultural institutions, journey through history, catching glimpses of our ancestors' stories.");
		},
		"id_9" => {
			cover = String::from("https://images.unsplash.com/photo-1559731131-e8d78734c858?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D");
			title = String::from("Experimental");
			description = String::from("With innovative techniques, unique perspectives, and unconventional subjects, this category is a playground for artistic exploration and boundary-defying creativity.");
		},
		"id_10" => {
			cover = String::from("https://images.unsplash.com/photo-1704241260682-5bb0a120cdaa?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D");
			title = String::from("Animals");
			description = String::from("Photographers capture the diversity, behavior, and beauty of creatures from across the globe, paying homage to the fascinating world of animals.");
		},
		"id_11" => {
			cover = String::from("https://images.unsplash.com/photo-1724313802311-da9f68e0875f?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D");
			title = String::from("Fashion & Beauty");
			description = String::from("Photography becomes a canvas for artistic expressions of fashion and beauty, capturing trends, styles, and personal statements in expertly composed images.");
		},
		"id_12" => {
			cover = String::from("https://images.unsplash.com/photo-1724433530860-f094e39b64e7?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D");
			title = String::from("People");
			description = String::from("Photographers depict the human experience, from candid moments to formal portraits, showcasing the myriad emotions, cultures, and stories that define us.");
		},
		"id_13" => {
			cover = String::from("https://images.unsplash.com/photo-1649194270591-8eead57b94c3?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D");
			title = String::from("Business & Work");
			description = String::from("Reflecting the realities of modern work — from remote working and start-ups to images of engineers and artists at work.");
		},
		"id_14" => {
			cover = String::from("https://images.unsplash.com/photo-1525480122447-64809d765ec4?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D");
			title = String::from("Food & Drink");
			description = String::from("Examining the world of food photography, from home-cooked dinners to tasting new dishes while traveling, capturing everything from picnics to decadent desserts.");
		},
		"id_15" => {
			cover = String::from("https://images.unsplash.com/photo-1633706961726-c67a0b691734?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D");
			title = String::from("Health & Wellness");
			description = String::from("Showcasing a healthy mind, body, and soul through photographs of medical discoveries, alternative medicines, healthy eating, and meditation.");
		},
		"id_16" => {
			cover = String::from("https://images.unsplash.com/photo-1604320239424-855007f7d759?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D");
			title = String::from("Sports");
			description = String::from("Celebrating sports photography, from adrenaline-fueled moments to camaraderie among athletes, highlighting the intensity and spirit of competition.");
		},
		_ => {}
	}

	Ok(Manga {
		id,
		cover,
		title,
		author,
		artist,
		description,
		url,
		categories,
		status,
		nsfw,
		viewer,
	})
}

#[get_chapter_list]
fn get_chapter_list(manga_id: String) -> Result<Vec<Chapter>> {
	let mut chapters: Vec<Chapter> = Vec::new();

	let id = format!("chapter_{}", manga_id);
	let title = String::from("View photos");
	let url = String::from("https://github.com/CaretReader/CaretApp");
	chapters.push(Chapter {
		id,
		title,
		url,
		..Default::default()
	});

	chapters.reverse();
	Ok(chapters)
}

#[get_page_list]
fn get_page_list(_: String, chapter_id: String) -> Result<Vec<Page>> {

	let mut pages: Vec<Page> = Vec::new();

	match chapter_id.as_str() {
		"chapter_id_1" => {
			let urls = [
				"https://images.unsplash.com/photo-1726065809961-bbfd67737399?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726517723307-c70c47791d1f?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1718202248333-1dab8271f400?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726441742327-746b1e0d431e?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1717699967168-9ccd317a0524?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1717397075045-8b4e0da1b41c?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1717831380980-5b1572491d5f?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1661880374682-7885c9432845?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1572816719298-4880240558a2?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726024708313-db80f6085141?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726092696603-002f851d9bc0?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726224437880-532a54c9565c?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1718017296470-4e34b11feb60?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1725652800358-ae3a752cfb68?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
			];

			for (i, url) in urls.iter().enumerate() {
				pages.push(Page {
					index: i as i32,
					url: url.to_string(),
					..Default::default()
				});
			}
		},
		"chapter_id_2" => {
			let urls = [
				"https://images.unsplash.com/photo-1726533815259-8fe320ac2493?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726480192181-83ad4f2310ad?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726452244167-ace3d3abe01a?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726551195923-346249e1c6b7?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726512978390-fa15979cc530?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726533828871-30bfcd8b84d7?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726538236643-2e50ca1ce8ea?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726554881162-ceeb7d68be8c?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726549962464-1dcabedf83b6?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726538236664-c25cb36e0642?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726510766668-94066aae6cd3?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726551195599-ab0f00e9c19b?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726486896376-4d1340e2f672?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726408093361-238693a8d51d?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
			];

			for (i, url) in urls.iter().enumerate() {
				pages.push(Page {
					index: i as i32,
					url: url.to_string(),
					..Default::default()
				});
			}
		},
		"chapter_id_3" => {
			let urls = [
				"https://images.unsplash.com/photo-1725113114010-978f258f6eeb?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1699009435420-756b3649564f?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1722224158349-9ad9829fd619?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1723478706124-c94a80b1afc4?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1724603142714-b5cac14f8bfe?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1725347740938-1c7b4b453847?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1717501219402-4444fcef55e7?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1721321559106-7ea5b41d084a?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1722691694088-b3b2ab29be31?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1722945629994-12ce3f1886ae?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
			];

			for (i, url) in urls.iter().enumerate() {
				pages.push(Page {
					index: i as i32,
					url: url.to_string(),
					..Default::default()
				});
			}
		},
		"chapter_id_4" => {
			let urls = [
				"https://images.unsplash.com/photo-1726711340800-d3709587de53?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1478436127897-769e1b3f0f36?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726346388471-5d0fd8dd051d?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726610930930-0e1af5f2d038?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726533862204-8110e6193fe9?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1725109431834-bed0465b6302?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726441344398-1c8b0953df71?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1597649260573-b3e9b226ecc8?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1571235722431-c2b865ecaa4b?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1683130461782-2a3f1910da18?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
			];

			for (i, url) in urls.iter().enumerate() {
				pages.push(Page {
					index: i as i32,
					url: url.to_string(),
					..Default::default()
				});
			}
		},
		"chapter_id_5" => {
			let urls = [
				"https://images.unsplash.com/photo-1687275862120-aef77a194565?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1700842240282-8db1cb01dbf4?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1714032857191-deda5932bcc1?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1683320277629-efa1dab7d828?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1684147212291-5fd3e9fc62aa?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1715611979979-35c3e21cfaf0?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726603855810-c67ffa6d2c2b?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726506413609-9b58dca4b080?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726466761043-fa4ef825a72f?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726445854917-f68225f52bf1?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
			];

			for (i, url) in urls.iter().enumerate() {
				pages.push(Page {
					index: i as i32,
					url: url.to_string(),
					..Default::default()
				});
			}
		},
		"chapter_id_6" => {
			let urls = [
				"https://images.unsplash.com/photo-1684057044985-6cb9a99b4663?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726597764443-561e60193f9d?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1513329941-ebee228697da?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726533878281-3212fbd99fc3?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1561212044-bac5ef688a07?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1612044706016-26c018b23f9c?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726413980384-6ff5bec9b948?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726417944449-0548bc9061a0?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726092691643-2a7923d36d87?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1725781762019-f7165f15282d?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
			];

			for (i, url) in urls.iter().enumerate() {
				pages.push(Page {
					index: i as i32,
					url: url.to_string(),
					..Default::default()
				});
			}
		},
		"chapter_id_7" => {
			let urls = [
				"https://images.unsplash.com/photo-1726498183842-cd39815e6782?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726498125587-8df23cc52a03?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726583736235-c270db8d5a3d?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726498136476-9cfd6201f8a0?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726499461664-8337f985e8c5?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726458573518-04a433641cb4?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726601057524-069373bb2fdf?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726601057282-2f1f1c13fa5f?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726593243881-847529d40d18?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726503454520-1de6e7420074?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
			];

			for (i, url) in urls.iter().enumerate() {
				pages.push(Page {
					index: i as i32,
					url: url.to_string(),
					..Default::default()
				});
			}
		},
		"chapter_id_8" => {
			let urls = [
				"https://images.unsplash.com/photo-1726409179221-9e4e486c9dbe?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1721852591222-89f60640fbb5?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1698943498584-73c6139ba93e?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1720033130610-45cac73d54a7?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1721853565038-d3dc10d9974d?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1712687947251-a6afed81d421?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1714331472376-eaad42afc1a6?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726409179258-1dbaa756de74?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1655125164706-22daff6a6661?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1655308422022-e0ec0ab9b0e2?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
			];

			for (i, url) in urls.iter().enumerate() {
				pages.push(Page {
					index: i as i32,
					url: url.to_string(),
					..Default::default()
				});
			}
		},
		"chapter_id_9" => {
			let urls = [
				"https://images.unsplash.com/photo-1585272190835-39703f145ad8?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726629280820-daab5e157be4?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726266002270-9a2ea2647051?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726575496082-ef6aa0d32b7f?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726551195795-612ca47c0b7d?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726497229519-203ca4e7e4f5?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726593243906-a89aa19cece7?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726619491583-9db69925639e?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1559731131-e8d78734c858?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1574388628724-c8319d449336?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1595933188697-3ed1690034c8?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
			];

			for (i, url) in urls.iter().enumerate() {
				pages.push(Page {
					index: i as i32,
					url: url.to_string(),
					..Default::default()
				});
			}
		},
		"chapter_id_10" => {
			let urls = [
				"https://images.unsplash.com/photo-1726687676612-c745a9ef9c21?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1661340583744-6d9e843aa363?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1604186838347-9faaf0b83be8?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1592355591640-4e3e558c6940?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1650109346561-85fec316cd55?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1498962911956-fea73daaa4c6?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1661340613834-9e6607787058?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1542317858-043bf4d34f9f?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1704241260682-5bb0a120cdaa?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726401579919-3cdd11d3555b?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
			];

			for (i, url) in urls.iter().enumerate() {
				pages.push(Page {
					index: i as i32,
					url: url.to_string(),
					..Default::default()
				});
			}
		},
		"chapter_id_11" => {
			let urls = [
				"https://images.unsplash.com/photo-1726725535296-99e5390399ea?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1664308703545-7c9eb05e920c?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1619466122087-e1ff06cf234b?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726591383725-59f0d79a6a5d?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726591383650-6f2740f79c9c?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726491703868-c74b074ffdcd?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726516335449-eaa942fd4c2f?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726556267339-b8af2ccbb2f7?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726349460485-3d6f2b9dfae4?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726211439062-5cc3797db5f5?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
			];

			for (i, url) in urls.iter().enumerate() {
				pages.push(Page {
					index: i as i32,
					url: url.to_string(),
					..Default::default()
				});
			}
		},
		"chapter_id_12" => {
			let urls = [
				"https://images.unsplash.com/photo-1726609196460-f2f6c9a04859?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726508271760-52f1744f2233?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726512078008-ac58bfd54342?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726508684402-ee6029833696?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726487646639-ec039193792f?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726495524246-00d680bbf389?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726251654985-b415579cd295?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1718152220071-dc4396f654fc?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1720535874037-a873d303ea75?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726262039919-a59235c634b2?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
			];

			for (i, url) in urls.iter().enumerate() {
				pages.push(Page {
					index: i as i32,
					url: url.to_string(),
					..Default::default()
				});
			}
		},
		"chapter_id_13" => {
			let urls = [
				"https://images.unsplash.com/photo-1470256699805-a29e1b58598a?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726421690313-2e0519335b82?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726298882745-f5010c72b107?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1504416285472-eccf03dd31eb?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1470256699805-a29e1b58598a?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1548783300-70b41bc84f56?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1511306162219-1c5a469ab86c?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1504720814069-969214e435c2?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726065235222-7d784fc40313?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1549488344-1f9b8d2bd1f3?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
			];

			for (i, url) in urls.iter().enumerate() {
				pages.push(Page {
					index: i as i32,
					url: url.to_string(),
					..Default::default()
				});
			}
		},
		"chapter_id_14" => {
			let urls = [
				"https://images.unsplash.com/photo-1726503453565-e9e1959908d5?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726519372376-6825c67bef15?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726576915247-144ee3df86bb?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726594220127-beca2cc920a1?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726127719239-2e6c15d9a8b7?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726575681677-6b9b3d51d0d4?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726563126219-17e602938e24?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726497864789-5c60f11e1d47?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726497864707-af6094549da0?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726513860423-ddde32db0898?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
			];

			for (i, url) in urls.iter().enumerate() {
				pages.push(Page {
					index: i as i32,
					url: url.to_string(),
					..Default::default()
				});
			}
		},
		"chapter_id_15" => {
			let urls = [
				"https://images.unsplash.com/photo-1685999298201-a49208ca8069?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1672015521020-ab4f86d5cc00?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1595978393470-41e725224986?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1722616683313-54a680aeddfe?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726711340800-d3709587de53?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1587093336587-eeca6cb17cf2?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1725799908052-52e7d899c7cd?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1726235812628-23558521686f?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1725137801173-1698d4cca0cd?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1717873642150-200b182955bc?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
			];

			for (i, url) in urls.iter().enumerate() {
				pages.push(Page {
					index: i as i32,
					url: url.to_string(),
					..Default::default()
				});
			}
		},
		"chapter_id_16" => {
			let urls = [
				"https://images.unsplash.com/photo-1723980839982-0c49951e7dba?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1724220432693-bb0b5c566dbb?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1724352012670-aae65f2bbd84?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1724451128160-6e6dc7c48a06?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1725199324454-7323473416c2?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1584129450613-8fde3f4bd426?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1571046629407-03dc7f13c305?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1589738373432-91e2d93453a4?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1624386753213-433d83bf6284?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
				"https://images.unsplash.com/photo-1725394806363-e7dd6e0bb8b8?q=90&w=1024&auto=jpg&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
			];

			for (i, url) in urls.iter().enumerate() {
				pages.push(Page {
					index: i as i32,
					url: url.to_string(),
					..Default::default()
				});
			}
		},
		_ => {}
	}

	Ok(pages)
}