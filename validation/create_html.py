def create_table(records,header):
    #header
    header_html = ''
    for item in header:
        header_html += f"<th>{item}</th>"

    if header_html != '':
        header_html = f"<tr>{header_html}</tr>"

    content_html = ''
    for item in records:
        line = ''
        for cell in item:
            line += f"<td>{cell}</td>"
        content_html += f"<tr>{line}</tr>"

    return f"<table>{header_html}{content_html}</table>"
