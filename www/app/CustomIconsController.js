define(['app'], function (app) {
	app.controller('CustomIconsController', ['$scope', '$rootScope', '$location', '$http', '$interval', function ($scope, $rootScope, $location, $http, $interval) {

		$scope.iconset = [];
		$scope.selectedIcon = [];

		$scope.uploadIcon = function (file) {
			var fd = new FormData();
			fd.append('file', file);
			$http.post('/domorust-api/custom_icons', fd, {
				transformRequest: angular.identity,
				headers: { 'Content-Type': undefined }
			}).then(function successCallback(response) {
				var data = response.data;
				if (data.status != "OK") {
					HideNotify();
					ShowNotify($.t('Error uploading Iconset') + ": " + data.error, 5000, true);
				}
				$scope.RefreshIconList();
			}, function errorCallback(response) {
				HideNotify();
				ShowNotify($.t('Error uploading Iconset'), 5000, true);
			});
		}

		$scope.UploadIconSet = function () {
			var file = $scope.myFile;
			if (typeof file == 'undefined') {
				HideNotify();
				ShowNotify($.t('Choose a File first!'), 2500, true);
				return;
			}
			$scope.uploadIcon(file);
		}

		$scope.RefreshIconList = function () {
			$scope.iconset = [];
			$scope.selectedIcon = [];

			$http({
				url: "domorust-api/custom_icons",
			}).then(function successCallback(response) {
				var data = response.data;
				if (typeof data.result != 'undefined') {
					$scope.iconset = data.result;
				}
			});
		}

		$scope.OnIconSelected = function (icon) {
			var bWasSelected = icon.selected;
			$.each($scope.iconset, function (i, item) {
				item.selected = false;
			});
			icon.selected = true;
			$scope.selectedIcon = icon;
		}

		$scope.UpdateIconTitleDescription = function () {
			var bValid = true;
			bValid = bValid && checkLength($("#iconname"), 2, 100);
			bValid = bValid && checkLength($("#icondescription"), 2, 100);
			if (bValid == false) {
				ShowNotify($.t('Please enter a Name and Description!...'), 3500, true);
				return;
			}
			$.ajax({
				method:"PUT",
				url: "domorust-api/custom_icons/" + $scope.selectedIcon.idx +
				'?name=' + encodeURIComponent($("#iconname").val()) +
				'&description=' + encodeURIComponent($("#icondescription").val()),
				async: false,
				dataType: 'json',
				success: function (data) {
					$scope.RefreshIconList();
				}
			});
		}

		$scope.DeleteIcon = function () {
			bootbox.confirm($.t("Are you sure to delete this Icon?"), function (result) {
				if (result == true) {
					$.ajax({
						method:"DELETE",
						url: "domorust-api/custom_icons/" + $scope.selectedIcon.idx,
						async: false,
						dataType: 'json',
						success: function (data) {
							$scope.RefreshIconList();
						}
					});
				}
			});
		}

		init();

		function init() {
			$('#iconsmain').i18n();
			$scope.RefreshIconList();
		};

	}]);
});