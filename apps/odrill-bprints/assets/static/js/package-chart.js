document.addEventListener('DOMContentLoaded', () => {
    const chartContainer = document.getElementById('downloadChart');
    if (!chartContainer) return;

    try {
        const history = JSON.parse(chartContainer.dataset.history);
        const dates = history.map(h => h.date);
        const values = history.map(h => h.downloads);

        const chart = echarts.init(chartContainer);

        const option = {
            backgroundColor: 'transparent',
            tooltip: {
                trigger: 'axis',
                backgroundColor: 'rgba(15, 23, 42, 0.9)',
                borderColor: 'rgba(148, 163, 184, 0.1)',
                textStyle: {
                    color: '#f1f5f9'
                }
            },
            grid: {
                left: '0%',
                right: '0%',
                bottom: '0%',
                top: '10%',
                containLabel: true
            },
            xAxis: {
                type: 'category',
                boundaryGap: false,
                data: dates,
                show: false // Hide X axis labels for cleaner look, tooltip handles it
            },
            yAxis: {
                type: 'value',
                splitLine: {
                    show: true,
                    lineStyle: {
                        color: 'rgba(148, 163, 184, 0.1)'
                    }
                },
                axisLabel: {
                    color: '#64748b'
                }
            },
            series: [
                {
                    name: 'Downloads',
                    type: 'line',
                    smooth: true,
                    symbol: 'none',
                    areaStyle: {
                        color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
                            {
                                offset: 0,
                                color: 'rgba(59, 130, 246, 0.5)'
                            },
                            {
                                offset: 1,
                                color: 'rgba(59, 130, 246, 0.01)'
                            }
                        ])
                    },
                    lineStyle: {
                        color: '#3b82f6',
                        width: 2
                    },
                    data: values
                }
            ]
        };

        chart.setOption(option);

        // Resize chart on window resize
        window.addEventListener('resize', () => {
            chart.resize();
        });

    } catch (e) {
        console.error("Error initializing ECharts: ", e);
    }
});
