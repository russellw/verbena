'<html lang="en">';
head {
	title {
		print TITLE;
	}
	style {
		body {
			// SORT
			'font-size' 20px;
			'font-family' 'Arial,sans-serif';
			display flex;
		}
		table {
			// SORT
			width '100%';
			'border-collapse' collapse;
		}
		th, td {
			// SORT
			border '1px solid #d3d3d3';
			padding 8px;
			'text-align' left;
		}
		th {
			'background-color' '#f2f2f2';
		}
	}
}

// body does not need a closing tag
'<body>';

// sidebar
div {
	@style {
		// SORT
		'background-color' '#000000';
		color '#ffffff';
		padding 20px;
	}
	h2 "Sales";
	ul {
		// TODO: li
		a {
			@href 'customers';
			"Customers";
		}
	}
}

#define contentStyle \
	@style { \
		padding 20px; \
	}
