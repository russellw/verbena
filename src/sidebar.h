'<html lang="en">';
head {
	title {
		print TITLE;
	}
	style {
		body {
			display flex;
			'font-family' 'Arial,sans-serif';
			'font-size' 20px;
		}
		table {
			'border-collapse' collapse;
			width '100%';
		}
		th, td {
			border '1px solid #d3d3d3';
			padding 8px;
			'text-align' left;
		}
		th {
			'background-color' '#f2f2f2';
		}
	}
}
'<body>';
div {
	@style {
		'background-color' 'lightgray';
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
