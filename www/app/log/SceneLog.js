define(['app', 'scenes/factories', 'log/components/DeviceTextLogTable'], function (app) {
	app.controller('SceneLogController', function ($scope, $routeParams, sceneApi) {
		var vm = this;

		vm.clearLog = clearLog;

		init();

		function init() {
			vm.autoRefresh = true;
			vm.sceneIdx = $routeParams.id;

			sceneApi.getSceneInfo(vm.sceneIdx).then(function (scene) {
				vm.pageName = scene.Name;
			});

			$scope.$on('scene_update', function(event, scene) {
				if (vm.autoRefresh && scene.idx === vm.sceneIdx) {
					refreshLog();
				}
			});

			refreshLog();
		}

		function refreshLog() {
			sceneApi.getSceneLog(vm.sceneIdx).then(function (data) {
				vm.log = data
			});
		}

		function clearLog() {
			bootbox.confirm($.t('Are you sure to delete the Log?\n\nThis action can not be undone!'), function (result) {
				if (result !== true) {
					return;
				}

				sceneApi.clearSceneLog(vm.sceneIdx)
					.then(refreshLog)
					.catch(function () {
						HideNotify();
						ShowNotify($.t('Problem clearing the Log!'), 2500, true);
					});
			});
		}
	});
});
