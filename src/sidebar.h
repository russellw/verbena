html {
	@lang en;
}
head {
	title {
		print TITLE;
	}
	style {
		// SORT
		'a:link' {
			color currentColor;
		}
		'a:visited' {
			color currentColor;
		}
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
		th {
			'background-color' '#f0f0f0';
		}
		th, td {
			// SORT
			border '1px solid #d0d0d0';
			padding 8px;
			'text-align' left;
		}
	}
}
body;

// sidebar
div {
	@style {
		// SORT
		'background-color' '#202020';
		color '#ffffff';
		padding 12px;
	}
	'Sales';
	ul {
		li {
			a {
				@href 'customers';
				'Customers';
			}
		}
	}
}

#define contentStyle \
	@style { \
		padding 12px; \
	}
