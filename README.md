.blz (Blaze) - int, str, date, time, datetime, bool, null, gap, list, set, obj
enum Gender (
	male = 1,
	female = 2,
	other = 3
);

@table accounts {
	$id: int,
	name: str (max=70) = "noName",
	gender: Gender?
};


@table texts {
	$id: int,
	content: str,
	created_at: datetime = "now",
	changes_count: int = 0,
	author: *"accounts.id"?;

	migrate("text", "code").to("content");
};

let years: int = 5;

.bq (Blaze Query) - new, set, get, del + variables and basic functions supported. 

get main.texts.{%author_id=12, regex_is_okay_for_kids(%content)}.content (
	group == %content, 
	filter={changes_count > 2}, 
	order_desc=%id,
	limit=10
); // 10 results


new main.texts.{
	[
		content="Hi! This is my first text on this platform. Please don't judge! So, ...",
		author: 123941
	],
	[
		content="Hello again! Sorry for my text was short, this will be more long and more informative as well. Let's ...",
		author: 123941
	]
};

del main.texts.{author == 123941} (limit=5, filter={%datetime == "recent"});
